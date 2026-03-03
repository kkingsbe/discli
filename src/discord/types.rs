//! Discord API type definitions

use serde::Serialize;
use std::path::PathBuf;

/// Represents a Discord message with optional attachments
#[derive(Debug, Clone)]
pub enum DiscordMessage {
    /// Simple text-only message
    Simple { content: String },
    /// Message with file attachments
    WithAttachments {
        content: Option<String>,
        attachments: Vec<FileAttachment>,
    },
    /// Message with embeds (rich embed support)
    WithEmbeds {
        content: Option<String>,
        embeds: Vec<Embed>,
    },
}

/// File attachment for Discord messages
#[derive(Debug, Clone)]
pub struct FileAttachment {
    /// Path to the file on the local filesystem
    pub path: PathBuf,
    /// Filename to use when uploading
    pub filename: String,
    /// MIME type of the file
    pub mime_type: String,
    /// File size in bytes
    pub size: u64,
    /// Optional description/alt text for the attachment
    pub description: Option<String>,
}

/// Discord embed structure with full field support
/// See Discord Developer Documentation for all available fields
#[derive(Debug, Clone, Serialize, Default)]
pub struct Embed {
    /// Title of the embed (max 256 characters)
    pub title: Option<String>,
    /// Description of the embed (max 4096 characters)
    pub description: Option<String>,
    /// URL of the embed (makes title clickable)
    pub url: Option<String>,
    /// Timestamp of the embed content (ISO 8601 format)
    pub timestamp: Option<String>,
    /// Color of the embed sidebar (decimal color value)
    pub color: Option<u32>,
    /// Footer information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Image to display in the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedMedia>,
    /// Thumbnail image in the top-right of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    /// Video to display (not typically used, requires proxy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,
    /// Provider information (e.g., website name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,
    /// Author information (displayed at top)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Fields to display in the embed (array, max 25)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,
}

/// Footer section of an embed
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedFooter {
    /// Footer text (max 2048 characters)
    pub text: String,
    /// URL of the footer icon (must be a valid URL)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Discord-proxied URL of the footer icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// Generic media (image) in an embed
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedMedia {
    /// URL of the media
    pub url: String,
    /// Discord-proxied URL of the media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// Height of the media (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width of the media (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// Thumbnail image in the embed
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedThumbnail {
    /// URL of the thumbnail
    pub url: String,
    /// Discord-proxied URL of the thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// Height of the thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width of the thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// Video in the embed
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedVideo {
    /// URL of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Discord-proxied URL of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,
    /// Height of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Width of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

/// Provider information (e.g., for embedded links)
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedProvider {
    /// Name of the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Author information displayed at the top of the embed
#[derive(Debug, Clone, Serialize, Default)]
pub struct EmbedAuthor {
    /// Name of the author (max 256 characters)
    pub name: String,
    /// URL of the author (makes name clickable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URL of the author icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Discord-proxied URL of the author icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// A single field in an embed
#[derive(Debug, Clone, Serialize)]
pub struct EmbedField {
    /// Name of the field (max 256 characters)
    pub name: String,
    /// Value of the field (max 1024 characters)
    pub value: String,
    /// Whether the field should be displayed inline
    #[serde(default)]
    pub inline: bool,
}

impl From<FileAttachment> for Attachment {
    fn from(file: FileAttachment) -> Self {
        Attachment {
            id: 0, // ID will be assigned by Discord
            description: file.description,
            filename: Some(file.filename),
        }
    }
}

/// Attachment metadata for Discord's payload_json
#[derive(Debug, Clone, Serialize)]
pub struct Attachment {
    pub id: u64,
    pub description: Option<String>,
    pub filename: Option<String>,
}
