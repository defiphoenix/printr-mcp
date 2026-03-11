use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SniperSettings {
    pub buy_amount: f64,
    pub max_market_cap: f64,
    pub min_liquidity: f64,
}

impl Default for SniperSettings {
    fn default() -> Self {
        Self {
            buy_amount: 0.1,
            max_market_cap: 50_000.0,
            min_liquidity: 10_000.0,
        }
    }
}
