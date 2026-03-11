use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub mint: String,
    pub symbol: String,
    pub market_cap: f64,
    pub liquidity: f64,
    pub initial_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenState {
    pub market_cap: f64,
    pub liquidity: f64,
    pub trade_count: u64,
    pub last_price: f64,
}
