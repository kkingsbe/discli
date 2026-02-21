//! Discord Gateway module
//!
//! This module provides WebSocket connectivity to the Discord Gateway,
//! allowing real-time event listening including MESSAGE_CREATE events.

use crate::error::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};
use twilight_gateway::{CloseFrame, Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt};
use twilight_model::gateway::payload::incoming::MessageCreate;

/// Discord Gateway client for receiving real-time events
///
/// This client connects to Discord's WebSocket Gateway and listens for events,
/// specifically MESSAGE_CREATE events. It handles reconnection automatically
/// with proper heartbeat handling.
pub struct DiscordGateway {
    /// The shard for the gateway connection
    shard: Shard,
    /// Discord bot token
    token: String,
    /// Flag to control shutdown
    shutdown_flag: Arc<std::sync::atomic::AtomicBool>,
    /// Event type flags for filtering
    event_flags: EventTypeFlags,
}

impl DiscordGateway {
    /// Create a new Gateway client
    ///
    /// # Arguments
    ///
    /// * `token` - Discord bot token
    ///
    /// # Returns
    ///
    /// A new `DiscordGateway` instance
    pub fn new(token: String) -> Self {
        let shutdown_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        // Create intents for message events
        let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;

        // Event types to listen for
        let event_flags = EventTypeFlags::MESSAGE_CREATE;

        // Create a shard with ID 0 (for small bots, one shard is sufficient)
        let shard = Shard::new(ShardId::ONE, token.clone(), intents);

        Self {
            shard,
            token,
            shutdown_flag,
            event_flags,
        }
    }

    /// Start the gateway and listen for events
    ///
    /// This method starts the WebSocket connection and runs the event loop,
    /// calling the provided handler for each MESSAGE_CREATE event.
    ///
    /// # Arguments
    ///
    /// * `handler` - Callback function to handle incoming messages
    ///
    /// # Returns
    ///
    /// `Ok(())` if the gateway runs successfully, or an error if connection fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to connect to the gateway
    /// - Connection is lost and reconnection fails
    pub async fn listen<F>(mut self, handler: F) -> Result<()>
    where
        F: Fn(MessageCreate) + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);
        let shutdown_flag = self.shutdown_flag.clone();

        // Process events from the shard
        // The shard auto-connects when we start polling
        info!("Starting Discord Gateway event loop...");

        while let Some(item) = self.shard.next_event(self.event_flags).await {
            // Check shutdown flag
            if shutdown_flag.load(std::sync::atomic::Ordering::Relaxed) {
                info!("Shutdown requested, stopping event loop");
                break;
            }

            let event = match item {
                Ok(event) => event,
                Err(source) => {
                    warn!("Error receiving event: {:?}", source);
                    continue;
                }
            };

            match event {
                Event::MessageCreate(msg) => {
                    info!(
                        "Received message from {} in channel {}",
                        msg.author.name, msg.channel_id
                    );
                    handler(*msg);
                }
                Event::Ready(ready) => {
                    info!(
                        "Gateway ready: {}#{} ({}), {} servers",
                        ready.user.name,
                        ready.user.discriminator,
                        ready.user.id,
                        ready.guilds.len()
                    );
                }
                Event::Resumed => {
                    info!("Gateway resumed");
                }
                _ => {
                    // Ignore other events for efficiency
                }
            }
        }

        info!("Gateway event loop ended");
        Ok(())
    }

    /// Get an event receiver channel for receiving events
    ///
    /// This provides an alternative to the callback-based approach,
    /// allowing the caller to receive events via a channel.
    ///
    /// # Returns
    ///
    /// A receiver for gateway events
    #[allow(dead_code)]
    pub fn events(&self) -> mpsc::Receiver<Event> {
        // Create a channel for events
        let (_tx, rx) = mpsc::channel(100);

        // Note: This is a simplified implementation
        // In production, you'd want to spawn a task that forwards events to the channel

        rx
    }

    /// Gracefully shutdown the gateway
    ///
    /// This stops the shard and closes connections cleanly.
    ///
    /// # Returns
    ///
    /// `Ok(())` if shutdown succeeds
    ///
    /// # Errors
    ///
    /// Returns an error if shutdown fails
    #[allow(dead_code)]
    pub async fn shutdown(&self) -> Result<()> {
        info!("Initiating gateway shutdown...");

        // Set the shutdown flag to stop the event loop
        self.shutdown_flag.store(true, std::sync::atomic::Ordering::Relaxed);

        // Close the shard connection
        self.shard.close(CloseFrame::NORMAL);

        info!("Gateway shutdown complete");
        Ok(())
    }

    /// Get the shard ID
    ///
    /// # Returns
    ///
    /// The shard ID
    #[allow(dead_code)]
    pub fn shard_id(&self) -> ShardId {
        self.shard.id()
    }
}

/// Create a new DiscordGateway instance
///
/// # Arguments
///
/// * `token` - Discord bot token
///
/// # Returns
///
/// A new `DiscordGateway` instance
#[allow(dead_code)]
pub fn create_gateway(token: String) -> DiscordGateway {
    DiscordGateway::new(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_creation() {
        let token = "test_token".to_string();
        let gateway = DiscordGateway::new(token.clone());
        assert_eq!(gateway.shard_id().number(), 1);
    }

    #[test]
    fn test_create_gateway() {
        let token = "test_token".to_string();
        let gateway = create_gateway(token);
        assert_eq!(gateway.shard_id().number(), 1);
    }
}
