//! Embed command implementation

use crate::config::Config;
use crate::discord::types::{DiscordMessage, Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedMedia, EmbedThumbnail};
use crate::discord::DiscordClient;
use crate::error::Result;

/// Execute the embed command
///
/// # Arguments
///
/// * `config` - Application configuration
/// * `title` - Embed title
/// * `embed_url` - URL for the title (makes it clickable)
/// * `description` - Embed description
/// * `color` - Embed color (hex string)
/// * `thumbnail` - Thumbnail image URL
/// * `image` - Main image URL
/// * `author` - Author name
/// * `author_icon` - Author icon URL
/// * `footer` - Footer text
/// * `footer_icon` - Footer icon URL
/// * `fields` - Fields to add (format: "name:value:inline")
/// * `content` - Optional text content above the embed
///
/// # Returns
///
/// `Ok(())` if message was sent successfully
pub async fn execute(
    config: &Config,
    title: Option<String>,
    embed_url: Option<String>,
    description: Option<String>,
    color: Option<String>,
    thumbnail: Option<String>,
    image: Option<String>,
    author: Option<String>,
    author_icon: Option<String>,
    footer: Option<String>,
    footer_icon: Option<String>,
    fields: Vec<String>,
    content: Option<String>,
) -> Result<()> {
    // Build the embed
    let mut embed = Embed::default();

    embed.title = title;
    embed.url = embed_url;
    embed.description = description;

    // Parse color (hex string like "FF5500" or "#FF5500")
    if let Some(color_str) = color {
        let color_val = parse_hex_color(&color_str)?;
        embed.color = Some(color_val);
    }

    // Set thumbnail
    if let Some(thumb_url) = thumbnail {
        embed.thumbnail = Some(EmbedThumbnail {
            url: thumb_url,
            ..Default::default()
        });
    }

    // Set main image
    if let Some(img_url) = image {
        embed.image = Some(EmbedMedia {
            url: img_url,
            ..Default::default()
        });
    }

    // Set author
    if let Some(author_name) = author {
        embed.author = Some(EmbedAuthor {
            name: author_name,
            icon_url: author_icon,
            ..Default::default()
        });
    }

    // Set footer
    if let Some(footer_text) = footer {
        embed.footer = Some(EmbedFooter {
            text: footer_text,
            icon_url: footer_icon,
            ..Default::default()
        });
    }

    // Parse fields
    for field_str in fields {
        if let Some(field) = parse_field(&field_str) {
            embed.fields.push(field);
        }
    }

    // Create Discord message with embed
    let discord_message = DiscordMessage::WithEmbeds {
        content,
        embeds: vec![embed],
    };

    // Send message
    let client = DiscordClient::new(config.discord_token.clone());
    client
        .send_message(&config.channel_id, &discord_message)
        .await?;

    println!("Successfully sent embed to channel {}", config.channel_id);

    Ok(())
}

/// Parse a hex color string to u32
fn parse_hex_color(color: &str) -> Result<u32> {
    let color = color.trim_start_matches('#');
    u32::from_str_radix(color, 16)
        .map_err(|_| crate::error::DiscliError::Validation(format!("Invalid hex color: {}", color)))
}

/// Parse a field string in format "name:value:inline"
fn parse_field(field_str: &str) -> Option<EmbedField> {
    let parts: Vec<&str> = field_str.splitn(3, ':').collect();
    if parts.len() < 2 {
        return None;
    }

    let name = parts[0].to_string();
    let value = parts[1].to_string();
    let inline = parts.get(2).map(|s| s.to_lowercase() == "true").unwrap_or(false);

    Some(EmbedField {
        name,
        value,
        inline,
    })
}
