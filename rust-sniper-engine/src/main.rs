mod commands;
mod engine;
mod models;
mod settings;
mod strategies;

use anyhow::Result;
use commands::command::Command;
use engine::engine::Engine;
use engine::event_bus::EventBus;
use log::info;
use models::events::{EngineEvent, NewTokenEvent, TradeEvent};
use settings::settings::SniperSettings;
use strategies::basic_sniper::BasicSniper;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let settings = SniperSettings::default();
    let (mut engine, event_bus) = Engine::new();

    // Register strategies that implement the Strategy trait. More can be added
    // here without changing engine internals.
    engine.register_strategy(Box::new(BasicSniper::new(settings)));

    let engine_handle = tokio::spawn(async move { engine.run().await });

    // Control the engine through commands.
    info!("[MAIN] Sending StartBot command");
    event_bus.send_command(Command::StartBot).await?;

    simulate_market_data(&event_bus).await?;

    // Demonstrate runtime settings update.
    let updated = SniperSettings {
        buy_amount: 0.25,
        max_market_cap: 75_000.0,
        min_liquidity: 8_000.0,
    };
    info!("[MAIN] Sending UpdateSettings command");
    event_bus
        .send_command(Command::UpdateSettings(updated))
        .await?;

    simulate_market_data(&event_bus).await?;

    info!("[MAIN] Sending StopBot command");
    event_bus.send_command(Command::StopBot).await?;

    // Give the engine loop time to process stop signal and drain channels.
    sleep(Duration::from_millis(200)).await;

    // Close channels by dropping bus and await engine shutdown.
    drop(event_bus);
    engine_handle.await??;

    Ok(())
}

async fn simulate_market_data(event_bus: &EventBus) -> Result<()> {
    let new_token = NewTokenEvent {
        mint: "TOKEN_ABC".to_string(),
        symbol: "ABC".to_string(),
        market_cap: 42_000.0,
        liquidity: 12_500.0,
        initial_price: 0.00042,
    };

    event_bus
        .publish(EngineEvent::NewToken(new_token.clone()))
        .await?;

    let trades = [
        TradeEvent {
            token_mint: new_token.mint.clone(),
            price: 0.00045,
            volume: 500.0,
            market_cap: 43_500.0,
            liquidity: 12_800.0,
        },
        TradeEvent {
            token_mint: new_token.mint,
            price: 0.00047,
            volume: 800.0,
            market_cap: 48_000.0,
            liquidity: 13_100.0,
        },
    ];

    for trade in trades {
        event_bus.publish(EngineEvent::Trade(trade)).await?;
        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
