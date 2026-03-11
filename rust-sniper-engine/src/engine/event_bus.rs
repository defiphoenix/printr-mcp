use anyhow::{Context, Result};
use tokio::sync::{broadcast, mpsc};

use crate::commands::command::Command;
use crate::models::events::EngineEvent;

/// Asynchronous event bus used by producers to publish market events and
/// operational commands into the engine.
#[derive(Clone)]
pub struct EventBus {
    event_tx: broadcast::Sender<EngineEvent>,
    command_tx: mpsc::Sender<Command>,
}

impl EventBus {
    pub fn new(
        event_tx: broadcast::Sender<EngineEvent>,
        command_tx: mpsc::Sender<Command>,
    ) -> Self {
        Self {
            event_tx,
            command_tx,
        }
    }

    /// Subscribe to market events.
    pub fn subscribe(&self) -> broadcast::Receiver<EngineEvent> {
        self.event_tx.subscribe()
    }

    pub async fn publish(&self, event: EngineEvent) -> Result<()> {
        self.event_tx
            .send(event)
            .map(|_| ())
            .context("failed to publish engine event")
    }

    pub async fn send_command(&self, command: Command) -> Result<()> {
        self.command_tx
            .send(command)
            .await
            .context("failed to send engine command")
    }
}
