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
    /// Message with embeds (future expansion)
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

/// Discord embed structure (for future expansion)
#[derive(Debug, Clone, Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
}

/// Image within an embed
#[derive(Debug, Clone, Serialize)]
pub struct EmbedImage {
    pub url: String,
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
