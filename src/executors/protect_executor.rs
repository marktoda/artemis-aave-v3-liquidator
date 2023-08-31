use std::{ops::Mul, sync::Arc};
use tracing::info;

use anyhow::{Context, Result};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::providers::Middleware;

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
}

impl<M: Middleware, N: Middleware> ProtectExecutor<M, N> {
    pub fn new(client: Arc<M>, sender_client: Arc<N>) -> Self {
        Self {
            client,
            sender_client,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempool> for ProtectExecutor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        info!("Executing tx {:?}", action.tx);
        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .context("Error estimating gas usage: {}");

        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result?;

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // old strategy - doesnt really make sense in l2 environment
            // // gas price at which we'd break even, meaning 100% of profit goes to validator
            // let breakeven_gas_price = gas_bid_info.total_profit / gas_usage;
            // // gas price corresponding to bid percentage
            // bid_gas_price = breakeven_gas_price
            //     .mul(gas_bid_info.bid_percentage)
            //     .div(100);
            // info!(
            //     "Gas bid info: {:?}, breakeven gas price: {}, bid gas price: {}",
            //     gas_bid_info, breakeven_gas_price, bid_gas_price
            // );

            // Just use estimated gas price but throw if its too low
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
            let estimated_cost = bid_gas_price.mul(gas_usage);
            if estimated_cost > gas_bid_info.total_profit {
                anyhow::bail!("Estimated cost of tx is greater than total profit");
            }
            info!(
                "Gas bid info: {:?}, estimated cost: {}, bid gas price: {}",
                gas_bid_info, estimated_cost, bid_gas_price
            );
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
        }
        action.tx.set_gas_price(bid_gas_price);
        self.sender_client.send_transaction(action.tx, None).await?;
        Ok(())
    }
}
