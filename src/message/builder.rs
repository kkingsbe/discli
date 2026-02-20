//! Message builder pattern

use crate::discord::types::{DiscordMessage, FileAttachment as DiscordFileAttachment};
use crate::error::Result;
use crate::message::FileAttachment;
use std::path::Path;

/// Builder for constructing Discord messages
///
/// This builder allows flexible construction of messages with:
/// - Text content
/// - File attachments
/// - Embeds (future expansion)
pub struct MessageBuilder {
    content: Option<String>,
    attachments: Vec<DiscordFileAttachment>,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> Self {
        Self {
            content: None,
            attachments: Vec::new(),
        }
    }

    /// Set the message content
    ///
    /// # Arguments
    ///
    /// * `content` - The text content of the message
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Add a file attachment to the message
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to attach
    ///
    /// # Returns
    ///
    /// The builder with the attachment added
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be loaded or validated
    pub fn add_attachment(mut self, path: &Path) -> Result<Self> {
        let attachment = FileAttachment::from_path(path)?;
        self.attachments.push(attachment.into());
        Ok(self)
    }

    /// Add multiple file attachments to the message
    ///
    /// # Arguments
    ///
    /// * `paths` - Iterator of paths to attach
    ///
    /// # Returns
    ///
    /// The builder with all attachments added
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be loaded or validated
    pub fn add_attachments<I, P>(mut self, paths: I) -> Result<Self>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        for path in paths {
            let attachment = FileAttachment::from_path(path.as_ref())?;
            self.attachments.push(attachment.into());
        }
        Ok(self)
    }

    /// Build the message into a DiscordMessage enum
    ///
    /// # Returns
    ///
    /// A DiscordMessage appropriate for the content type
    pub fn build(self) -> DiscordMessage {
        if self.attachments.is_empty() {
            DiscordMessage::Simple {
                content: self.content.unwrap_or_default(),
            }
        } else {
            DiscordMessage::WithAttachments {
                content: self.content,
                attachments: self.attachments,
            }
        }
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_simple_message() {
        let builder = MessageBuilder::new().content("Hello, World!");
        let message = builder.build();
        assert!(matches!(message, DiscordMessage::Simple { content } if content == "Hello, World!"));
    }

    #[test]
    fn test_builder_empty_message() {
        let builder = MessageBuilder::new();
        let message = builder.build();
        assert!(matches!(message, DiscordMessage::Simple { content } if content.is_empty()));
    }

    #[test]
    fn test_builder_default() {
        let builder = MessageBuilder::default();
        assert_eq!(builder.content, None);
        assert_eq!(builder.attachments.len(), 0);
    }
}
