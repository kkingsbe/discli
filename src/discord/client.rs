//! Discord API client

use crate::discord::api::{send_json_message, send_multipart_message};
use crate::discord::types::DiscordMessage;
use crate::error::Result;
use reqwest::Client;

/// Discord API client for sending messages
pub struct DiscordClient {
    /// HTTP client for making API requests
    http_client: Client,
    /// Discord bot token
    token: String,
    /// Base URL for Discord API
    base_url: String,
}

impl DiscordClient {
    /// Create a new Discord API client
    ///
    /// # Arguments
    ///
    /// * `token` - Discord bot token
    ///
    /// # Returns
    ///
    /// A new `DiscordClient` instance
    pub fn new(token: String) -> Self {
        let http_client = Client::new();
        Self {
            http_client,
            token,
            base_url: "https://discord.com/api/v10".to_string(),
        }
    }

    /// Send a message to a Discord channel
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Discord channel ID to send message to
    /// * `message` - The message to send
    ///
    /// # Returns
    ///
    /// `Ok(())` if the message was sent successfully
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails
    /// - Discord returns an error response
    /// - Invalid channel ID or token
    pub async fn send_message(
        &self,
        channel_id: &str,
        message: &DiscordMessage,
    ) -> Result<()> {
        let url = format!("{}/channels/{}/messages", self.base_url, channel_id);

        match message {
            DiscordMessage::Simple { content } => {
                send_json_message(&self.http_client, &url, &self.token, content).await
            }
            DiscordMessage::WithAttachments {
                content,
                attachments,
            } => {
                send_multipart_message(&self.http_client, &url, &self.token, content, attachments)
                    .await
            }
            DiscordMessage::WithEmbeds { content: _content, embeds: _embeds } => {
                // TODO: Implement embed support
                Err(crate::error::DiscliError::DiscordApi(
                    "Embed support not yet implemented".into(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let token = "test_token".to_string();
        let client = DiscordClient::new(token);
        assert_eq!(client.token, "test_token");
        assert_eq!(client.base_url, "https://discord.com/api/v10");
    }
}
