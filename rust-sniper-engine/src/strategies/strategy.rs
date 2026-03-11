use crate::models::events::TradeEvent;
use crate::models::order::OrderRequest;
use crate::models::token::TokenInfo;
use crate::settings::settings::SniperSettings;

/// Strategy Pattern abstraction.
///
/// Any future strategy (PumpFun sniper, migration sniper, copy trader, etc.)
/// only needs to implement this trait and can be plugged into the engine.
pub trait Strategy {
    fn name(&self) -> &'static str;
    fn on_trade(&mut self, trade: &TradeEvent) -> Option<OrderRequest>;
    fn on_new_token(&mut self, token: &TokenInfo) -> Option<OrderRequest>;

    fn update_settings(&mut self, _settings: &SniperSettings) {
        // Default no-op; strategies can override.
    }
}
