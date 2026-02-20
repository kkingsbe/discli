//! File attachment handling

use crate::discord::types::FileAttachment as DiscordFileAttachment;
use crate::error::{DiscliError, Result};
use mime_guess;
use std::path::{Path, PathBuf};

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

impl FileAttachment {
    /// Create a new FileAttachment from a file path
    ///
    /// This function validates that:
    /// - The file exists
    /// - The file size doesn't exceed Discord's 25MB limit
    /// - The file has a valid image MIME type
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Returns
    ///
    /// A new `FileAttachment` instance
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file doesn't exist
    /// - The file size exceeds 25MB
    /// - The MIME type is not an image
    /// - The filename is invalid
    pub fn from_path(path: &Path) -> Result<Self> {
        // Check file exists
        if !path.exists() {
            return Err(DiscliError::Attachment(format!(
                "File not found: {}",
                path.display()
            )));
        }

        // Get file metadata
        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();

        // Validate size
        if size > 25 * 1024 * 1024 {
            return Err(DiscliError::Attachment(
                "File exceeds Discord's 25MB limit".into(),
            ));
        }

        // Determine filename
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DiscliError::Attachment("Invalid filename".into()))?
            .to_string();

        // Detect MIME type
        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        // Validate it's an image (basic check)
        if !mime_type.starts_with("image/") {
            return Err(DiscliError::Attachment(format!(
                "Not an image file: {} (detected type: {})",
                path.display(),
                mime_type
            )));
        }

        Ok(Self {
            path: path.to_path_buf(),
            filename,
            mime_type,
            size,
            description: None,
        })
    }

    /// Set a description for this attachment
    ///
    /// # Arguments
    ///
    /// * `description` - Description/alt text for the attachment
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl From<FileAttachment> for DiscordFileAttachment {
    fn from(file: FileAttachment) -> Self {
        DiscordFileAttachment {
            path: file.path,
            filename: file.filename,
            mime_type: file.mime_type,
            size: file.size,
            description: file.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires actual file
    fn test_attachment_from_path() {
        // This test requires an actual file to exist
        // Create a test file before running
    }

    #[test]
    fn test_with_description() {
        let attachment = FileAttachment {
            path: PathBuf::from("/fake/path.png"),
            filename: "test.png".to_string(),
            mime_type: "image/png".to_string(),
            size: 1000,
            description: None,
        };

        let with_desc = attachment.with_description("Test description".to_string());
        assert_eq!(with_desc.description, Some("Test description".to_string()));
    }
}
