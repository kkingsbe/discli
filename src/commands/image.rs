//! Image command implementation

use crate::config::Config;
use crate::error::Result;
use std::path::PathBuf;

/// Execute the image command
///
/// This is a convenience command that focuses on sending images.
/// It reuses the send command's logic but emphasizes the image aspect.
///
/// # Arguments
///
/// * `config` - Application configuration
/// * `attach` - List of file paths to attach (at least one required)
/// * `caption` - Optional caption text for the images
/// * `embed_url` - List of image URLs to embed (future feature)
///
/// # Returns
///
/// `Ok(())` if message was sent successfully
pub async fn execute(
    config: &Config,
    attach: Vec<PathBuf>,
    caption: Option<String>,
    embed_url: Vec<String>,
) -> Result<()> {
    // Use the caption as the content (or empty string if no caption)
    let content = caption.unwrap_or_default();

    // Reuse the send command's logic
    super::send::execute(config, content, attach, embed_url, None).await
}
