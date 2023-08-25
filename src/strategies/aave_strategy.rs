use super::types::Config;
use crate::collectors::{block_collector::NewBlock, time_collector::NewTick};
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_aave::{
    i_aave_oracle::IAaveOracle,
    i_pool_data_provider::IPoolDataProvider,
    pool::{BorrowFilter, Pool, SupplyFilter},
};
use ethers::{
    abi::{ethabi, AbiEncode, Token},
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, Filter, H160, U256, U64},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use tracing::{error, info};

use super::types::{Action, Event};

const BLOCK_TIME: u64 = 12;
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const POOL_ADDRESS: &str = "0xA238Dd80C259a72e81d7e4664a9801593F98d1c5";
pub const POOL_DATA_PROVIDER: &str = "0x2d8A3C5677189723C4cB8873CfC9C8976FDF38Ac";
pub const AAVE_ORACLE_ADDRESS: &str = "0x2Cc0Fc26eD4563A5ce5e8bdcfe1A2878676Ae156";
pub const POOL_CREATION_BLOCK: u64 = 2963358;
pub const LOG_BLOCK_RANGE: u64 = 1024;
pub const STATE_CACHE_FILE: &str = "borrowers.json";

pub const LIQUIDATION_CLOSE_FACTOR_THRESHOLD: &str = "950000000000000000";
pub const MAX_LIQUIDATION_CLOSE_FACTOR: u64 = 10000;
pub const DEFAULT_LIQUIDATION_CLOSE_FACTOR: u64 = 5000;

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCache {
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Borrower {
    address: Address,
    collateral: HashSet<Address>,
    debt: HashSet<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    address: Address,
    decimals: u64,
    ltv: u64,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
    reserve_factor: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AaveStrategy<M> {
    /// Ethers client.
    client: Arc<M>,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
    last_block_number: u64,
    borrowers: HashMap<Address, Borrower>,
    tokens: HashMap<Address, TokenConfig>,
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    pub fn new(client: Arc<M>, config: Config) -> Self {
        Self {
            client,
            bid_percentage: config.bid_percentage,
            last_block_number: 0,
            borrowers: HashMap::new(),
            tokens: HashMap::new(),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for AaveStrategy<M> {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        self.update_token_configs().await?;
        self.load_cache()?;
        self.update_state().await?;

        info!("done syncing state");
        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            // Event::NewBlock(block) => self.process_new_block_event(block).await,
            Event::NewTick(block) => self.process_new_tick_event(block).await,
        }
    }
}

impl<M: Middleware + 'static> AaveStrategy<M> {
    /// Process new block events, updating the internal state.
    // async fn process_new_block_event(&mut self, event: NewBlock) -> Option<Action> {
    //     info!("received new block: {:?}", event);
    //     self.last_block_number = event.number.as_u64();
    //     None
    // }

    /// Process new block events, updating the internal state.
    async fn process_new_tick_event(&mut self, event: NewTick) -> Option<Action> {
        info!("received new tick: {:?}", event);
        self.update_state()
            .await
            .map_err(|e| error!("Update State error: {}", e))
            .ok()?;

        info!("Total borrower count: {}", self.borrowers.len());
        let underwater = self
            .get_underwater_borrowers()
            .await
            .map_err(|e| error!("Get underwater borrowers error: {}", e))
            .ok()?;

        if underwater.len() == 0 {
            info!("No underwater borrowers found");
            return None;
        }

        info!("Found {} underwater borrowers", underwater.len());
        let pool_data = IPoolDataProvider::<M>::new(
            POOL_DATA_PROVIDER.parse::<Address>().unwrap(),
            self.client.clone(),
        );
        for (borrower, health_factor) in underwater {
            let Borrower {
                address: _,
                collateral,
                debt,
            } = self.borrowers.get(&borrower)?;
            // TODO: handle users with multiple collateral / debt
            // get first item out of the set
            let collateral_address = collateral.iter().next()?;
            let debt_address = debt.iter().next()?;

            let (_, stable_debt, variable_debt, _, _, _, _, _, _) = pool_data
                .get_user_reserve_data(*debt_address, borrower)
                .await
                .map_err(|e| error!("{:?}", e))
                .ok()?;
            let collateral_reserve = pool_data
                .get_user_reserve_data(*collateral_address, borrower)
                .await
                .map_err(|e| error!("{:?}", e))
                .ok()?;
            let close_factor = if health_factor.lt(&U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)) {
                U256::from(DEFAULT_LIQUIDATION_CLOSE_FACTOR)
            } else {
                U256::from(MAX_LIQUIDATION_CLOSE_FACTOR)
            };
            let TokenConfig { liquidation_bonus, .. } = self.tokens.get(collateral_address)?;
            info!("collateral reserve: {:?}", collateral_reserve);
            let debt_to_cover = (stable_debt + variable_debt) * close_factor;
            info!("debt to cover: {:?}", debt_to_cover);
            let max_collateral_to_liquidate =
                (self.get_asset_price(debt_address).await.ok()? * debt_to_cover * liquidation_bonus)
                    / self.get_asset_price(collateral_address).await.ok()?;
            info!(
                "max collateral to liquidate: {:?}",
                max_collateral_to_liquidate
            );
        }

        None
    }

    // for all known borrowers, return a sorted set of those with health factor < 1
    async fn get_underwater_borrowers(&mut self) -> Result<Vec<(Address, U256)>> {
        let pool = Pool::<M>::new(
            POOL_ADDRESS.parse::<Address>().unwrap(),
            self.client.clone(),
        );

        let mut underwater_borrowers = Vec::new();

        // call pool.getUserAccountData(user) for each borrower
        for borrower in self.borrowers.values() {
            if borrower.debt.len() == 0 {
                continue;
            }

            let user_account_data = pool.get_user_account_data(borrower.address).await?;
            let health_factor = user_account_data.5;
            if health_factor.lt(&U256::from_dec_str("1000000000000000000").unwrap()) {
                info!(
                    "Found underwater borrower {:?} -  healthFactor: {}",
                    borrower, health_factor
                );
                underwater_borrowers.push((borrower.address, health_factor));
            }
        }

        // sort borrowers by health factor
        underwater_borrowers.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(underwater_borrowers)
    }

    // load borrower state cache from file if exists
    fn load_cache(&mut self) -> Result<()> {
        match File::open(STATE_CACHE_FILE) {
            Ok(file) => {
                let cache: StateCache = serde_json::from_reader(file)?;
                info!("read state cache from file");
                self.last_block_number = cache.last_block_number;
                self.borrowers = cache.borrowers;
            }
            Err(_) => {
                info!("no state cache file found, creating new one");
                self.last_block_number = POOL_CREATION_BLOCK;
            }
        };

        Ok(())
    }

    // update known borrower state from last block to latest block
    async fn update_state(&mut self) -> Result<()> {
        let latest_block = self.client.get_block_number().await?;
        info!(
            "Updating state from block {} to {}",
            self.last_block_number, latest_block
        );

        self.get_borrow_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                // fetch assets if user already a borrower
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.debt.insert(log.reserve);
                    return;
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            collateral: HashSet::new(),
                            debt: HashSet::from([log.reserve]),
                        },
                    );
                }
            });

        self.get_supply_logs(self.last_block_number.into(), latest_block)
            .await?
            .into_iter()
            .for_each(|log| {
                let user = log.on_behalf_of;
                // fetch assets if user already a supplier
                if self.borrowers.contains_key(&user) {
                    let borrower = self.borrowers.get_mut(&user).unwrap();
                    borrower.collateral.insert(log.reserve);
                    return;
                } else {
                    self.borrowers.insert(
                        user,
                        Borrower {
                            address: user,
                            collateral: HashSet::from([log.reserve]),
                            debt: HashSet::new(),
                        },
                    );
                }
            });

        // write state cache to file
        let cache = StateCache {
            last_block_number: latest_block.as_u64(),
            borrowers: self.borrowers.clone(),
        };
        self.last_block_number = latest_block.as_u64();
        let mut file = File::create(STATE_CACHE_FILE)?;
        file.write_all(serde_json::to_string(&cache)?.as_bytes())?;

        Ok(())
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_borrow_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<BorrowFilter>> {
        let pool = Pool::<M>::new(
            POOL_ADDRESS.parse::<Address>().unwrap(),
            self.client.clone(),
        );

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.borrow_filter()
                .from_block(start_block)
                .to_block(end_block)
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    // fetch all borrow events from the from_block to to_block
    async fn get_supply_logs(&self, from_block: U64, to_block: U64) -> Result<Vec<SupplyFilter>> {
        let pool = Pool::<M>::new(
            POOL_ADDRESS.parse::<Address>().unwrap(),
            self.client.clone(),
        );

        let mut res = Vec::new();
        for start_block in
            (from_block.as_u64()..to_block.as_u64()).step_by(LOG_BLOCK_RANGE as usize)
        {
            let end_block = std::cmp::min(start_block + LOG_BLOCK_RANGE - 1, to_block.as_u64());
            pool.supply_filter()
                .from_block(start_block)
                .to_block(end_block)
                .query()
                .await?
                .into_iter()
                .for_each(|log| {
                    res.push(log);
                });
        }

        Ok(res)
    }

    async fn update_token_configs(&mut self) -> Result<()> {
        let pool_data = IPoolDataProvider::<M>::new(
            POOL_DATA_PROVIDER.parse::<Address>().unwrap(),
            self.client.clone(),
        );
        let all_tokens = pool_data.get_all_reserves_tokens().await?;
        info!("all_tokens: {:?}", all_tokens);
        for token in all_tokens {
            let (decimals, ltv, threshold, bonus, reserve, _, _, _, _, _) = pool_data
                .get_reserve_configuration_data(token.token_address)
                .await?;
            self.tokens.insert(
                token.token_address,
                TokenConfig {
                    address: token.token_address,
                    decimals: decimals.low_u64(),
                    ltv: ltv.low_u64(),
                    liquidation_threshold: threshold.low_u64(),
                    liquidation_bonus: bonus.low_u64(),
                    reserve_factor: reserve.low_u64(),
                },
            );
        }

        Ok(())
    }

    async fn get_asset_price(&self, asset: &Address) -> Result<U256> {
        let price_oracle = IAaveOracle::<M>::new(
            AAVE_ORACLE_ADDRESS.parse::<Address>().unwrap(),
            self.client.clone(),
        );
        let price = price_oracle.get_asset_price(*asset).await?;
        Ok(price)
    }
}
