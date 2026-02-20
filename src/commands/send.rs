//! Send command implementation

use crate::config::Config;
use crate::discord::DiscordClient;
use crate::error::Result;
use crate::message::MessageBuilder;
use std::path::PathBuf;

/// Execute the send command
///
/// # Arguments
///
/// * `config` - Application configuration
/// * `content` - Message content to send
/// * `attach` - List of file paths to attach
/// * `embed_url` - List of image URLs to embed (future feature)
/// * `caption` - Optional caption/description for attachments
///
/// # Returns
///
/// `Ok(())` if message was sent successfully
pub async fn execute(
    config: &Config,
    content: String,
    attach: Vec<PathBuf>,
    _embed_url: Vec<String>,
    _caption: Option<String>,
) -> Result<()> {
    // Validate attachment count (files + URLs)
    crate::message::validation::validate_attachment_count(attach.len() + _embed_url.len())?;

    // Validate content length if present
    if !content.is_empty() {
        crate::message::validation::validate_content_length(&content)?;
    }

    // Build message
    let mut builder = MessageBuilder::new();

    // Add content
    if !content.is_empty() {
        builder = builder.content(content);
    }

    // Add file attachments
    for path in &attach {
        builder = builder.add_attachment(path)?;
    }

    // Note: embed_url support will be added in future expansion
    // For now, we only support file uploads

    // Build the Discord message
    let discord_message = builder.build();

    // Send message
    let client = DiscordClient::new(config.discord_token.clone());
    client
        .send_message(&config.channel_id, &discord_message)
        .await?;

    // Print success message
    let summary = if attach.is_empty() {
        format!("text message")
    } else {
        format!("message with {} image attachment(s)", attach.len())
    };
    println!("Successfully sent {} to channel {}", summary, config.channel_id);

    Ok(())
}
