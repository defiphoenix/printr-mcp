use log::debug;

use crate::models::events::TradeEvent;
use crate::models::order::{OrderRequest, OrderSide};
use crate::models::token::TokenInfo;
use crate::settings::settings::SniperSettings;
use crate::strategies::strategy::Strategy;

pub struct BasicSniper {
    settings: SniperSettings,
}

impl BasicSniper {
    pub fn new(settings: SniperSettings) -> Self {
        Self { settings }
    }

    fn build_buy_order(&self, token_mint: String) -> OrderRequest {
        OrderRequest {
            strategy_name: self.name().to_string(),
            token_mint,
            side: OrderSide::Buy,
            amount_sol: self.settings.buy_amount,
        }
    }

    fn qualifies(&self, market_cap: f64, liquidity: f64) -> bool {
        market_cap < self.settings.max_market_cap && liquidity > self.settings.min_liquidity
    }
}

impl Strategy for BasicSniper {
    fn name(&self) -> &'static str {
        "basic_sniper"
    }

    fn on_trade(&mut self, trade: &TradeEvent) -> Option<OrderRequest> {
        if self.qualifies(trade.market_cap, trade.liquidity) {
            return Some(self.build_buy_order(trade.token_mint.clone()));
        }

        debug!(
            "[BASIC_SNIPER] Trade ignored: mc={} liq={}",
            trade.market_cap, trade.liquidity
        );
        None
    }

    fn on_new_token(&mut self, token: &TokenInfo) -> Option<OrderRequest> {
        if self.qualifies(token.market_cap, token.liquidity) {
            return Some(self.build_buy_order(token.mint.clone()));
        }

        debug!(
            "[BASIC_SNIPER] New token ignored: mc={} liq={}",
            token.market_cap, token.liquidity
        );
        None
    }

    fn update_settings(&mut self, settings: &SniperSettings) {
        self.settings = settings.clone();
    }
}
