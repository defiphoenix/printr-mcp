use dashmap::DashMap;

use crate::models::events::{NewTokenEvent, TradeEvent};
use crate::models::token::TokenState;

/// Shared in-memory engine state. DashMap allows lock-sharded concurrent access.
pub type TokenStore = DashMap<String, TokenState>;

pub fn apply_new_token(store: &TokenStore, event: &NewTokenEvent) {
    store.insert(
        event.mint.clone(),
        TokenState {
            market_cap: event.market_cap,
            liquidity: event.liquidity,
            trade_count: 0,
            last_price: event.initial_price,
        },
    );
}

pub fn apply_trade(store: &TokenStore, event: &TradeEvent) {
    if let Some(mut token_state) = store.get_mut(&event.token_mint) {
        token_state.market_cap = event.market_cap;
        token_state.liquidity = event.liquidity;
        token_state.last_price = event.price;
        token_state.trade_count += 1;
        return;
    }

    // Fallback: initialize state if trade arrives before token listing event.
    store.insert(
        event.token_mint.clone(),
        TokenState {
            market_cap: event.market_cap,
            liquidity: event.liquidity,
            trade_count: 1,
            last_price: event.price,
        },
    );
}
