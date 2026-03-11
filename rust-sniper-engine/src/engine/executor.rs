use log::info;

use crate::engine::error::ExecutionError;
use crate::models::order::{OrderRequest, OrderSide};

#[derive(Default)]
pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, order: OrderRequest) -> Result<(), ExecutionError> {
        let side = match order.side {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        };

        info!(
            "[EXECUTOR] Executing {side} order for {} amount {} SOL (strategy={})",
            order.token_mint, order.amount_sol, order.strategy_name
        );

        Ok(())
    }
}
