use serde::{Deserialize, Serialize};

use crate::models::token::TokenInfo;
use crate::models::trade::Trade;

pub type TradeEvent = Trade;
pub type NewTokenEvent = TokenInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineEvent {
    Trade(TradeEvent),
    NewToken(NewTokenEvent),
}
