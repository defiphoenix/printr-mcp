use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub token_mint: String,
    pub price: f64,
    pub volume: f64,
    pub market_cap: f64,
    pub liquidity: f64,
}
