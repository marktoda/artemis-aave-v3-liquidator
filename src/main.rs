use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::{block_collector::BlockCollector, time_collector::TimeCollector};
use ethers::{
    prelude::MiddlewareBuilder,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use executors::protect_executor::ProtectExecutor;
use std::sync::Arc;
use strategies::{
    aave_strategy::AaveStrategy,
    types::{Action, Config, Event},
};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

pub mod collectors;
pub mod executors;
pub mod strategies;

static POLL_INTERVAL_SECS: u64 = 60;
pub const CHAIN_ID: u64 = 8453;

const MEV_BLOCKER: &str = "https://rpc.mevblocker.io/noreverts";

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub rpc: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    /// Percentage of profit to pay in gas.
    #[arg(long)]
    pub bid_percentage: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("aave_v3_liquidator", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let rpc = Http::from_str(&args.rpc)?;
    let provider = Provider::new(rpc);

    let mevblocker_provider =
        Provider::<Http>::try_from(MEV_BLOCKER).expect("could not instantiate HTTP Provider");

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(CHAIN_ID);
    let address = wallet.address();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));
    let mevblocker_provider = Arc::new(
        mevblocker_provider
            .nonce_manager(address)
            .with_signer(wallet),
    );

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up block collector.
    // let block_collector = Box::new(BlockCollector::new(provider.clone()));
    // let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    // engine.add_collector(Box::new(block_collector));

    // Set up time collector.
    let time_collector = Box::new(TimeCollector::new(POLL_INTERVAL_SECS));
    let time_collector = CollectorMap::new(time_collector, Event::NewTick);
    engine.add_collector(Box::new(time_collector));

    let config = Config {
        bid_percentage: args.bid_percentage,
    };

    let strategy = AaveStrategy::new(Arc::new(provider.clone()), config);
    engine.add_strategy(Box::new(strategy));

    let executor = Box::new(ProtectExecutor::new(
        provider.clone(),
        mevblocker_provider.clone(),
    ));

    let executor = ExecutorMap::new(executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
    });

    engine.add_executor(Box::new(executor));
    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }
    Ok(())
}
