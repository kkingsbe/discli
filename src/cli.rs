//! CLI argument definitions using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool for sending Discord notifications with image support
#[derive(Parser)]
#[command(name = "discli")]
#[command(about = "Send messages and images to Discord from the command line", long_about = None)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Legacy: direct message argument (for backward compatibility)
    ///
    /// DEPRECATED: Use 'discli send' instead
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub legacy_message: Vec<String>,
}

/// Available subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Send a message to Discord
    ///
    /// This is the primary command for sending messages. It can send:
    /// - Plain text messages
    /// - Text with image attachments
    /// - Images only (no text)
    #[command(alias = "message")]
    Send {
        /// Message content to send
        ///
        /// If not provided, sends images only (if --attach is used)
        #[arg(required = false)]
        content: String,

        /// Attach image files (can be specified multiple times)
        ///
        /// Supported formats: PNG, JPG, GIF, WebP, etc.
        #[arg(short, long, value_name = "PATH")]
        attach: Vec<PathBuf>,

        /// Embed image URLs (can be specified multiple times)
        ///
        /// Embed externally hosted images without uploading them
        #[arg(long, value_name = "URL")]
        embed_url: Vec<String>,

        /// Alt text/description for attachments
        ///
        /// This applies to all attachments
        #[arg(short, long, value_name = "TEXT")]
        caption: Option<String>,
    },

    /// Send a message with images (convenience command)
    ///
    /// This is a convenience alias for 'discli send' that focuses on images.
    /// Images are required, text is optional.
    Image {
        /// Image files to attach (can be specified multiple times)
        ///
        /// At least one image is required
        #[arg(short, long, required = true, value_name = "PATH")]
        attach: Vec<PathBuf>,

        /// Caption text for images
        ///
        /// Optional text to send with the images
        #[arg(short, long, value_name = "TEXT")]
        caption: Option<String>,

        /// Embed image URLs instead of uploading
        ///
        /// Embed externally hosted images without uploading them
        #[arg(long, value_name = "URL")]
        embed_url: Vec<String>,
    },

    /// Send a rich embed message
    ///
    /// Create visually rich Discord embeds with custom styling.
    /// Perfect for recipes, notifications, and formatted messages.
    Embed {
        /// Title of the embed (max 256 chars)
        ///
        /// This appears at the top of the embed and can be a clickable link with --embed-url
        #[arg(long, value_name = "TEXT")]
        title: Option<String>,

        /// URL for the embed title (makes title clickable)
        #[arg(long, value_name = "URL")]
        embed_url: Option<String>,

        /// Description text (max 4096 chars)
        ///
        /// Main content of the embed below the title
        #[arg(long, value_name = "TEXT")]
        description: Option<String>,

        /// Embed color as hex (e.g., FF5500 or #FF5500)
        ///
        /// Color of the sidebar in the embed
        #[arg(long, value_name = "HEX")]
        color: Option<String>,

        /// Thumbnail image URL (shown in top-right)
        #[arg(long, value_name = "URL")]
        thumbnail: Option<String>,

        /// Main image URL (shown below description)
        #[arg(long, value_name = "URL")]
        image: Option<String>,

        /// Author name (displayed at top)
        #[arg(long, value_name = "TEXT")]
        author: Option<String>,

        /// Author icon URL
        #[arg(long, value_name = "URL")]
        author_icon: Option<String>,

        /// Footer text
        #[arg(long, value_name = "TEXT")]
        footer: Option<String>,

        /// Footer icon URL
        #[arg(long, value_name = "URL")]
        footer_icon: Option<String>,

        /// Add a field (can be repeated, format: "name:value:inline")
        ///
        /// Use 'true' or 'false' for inline parameter
        /// Example: --field "Ingredients:item1,item2:true"
        #[arg(long, value_name = "NAME:VALUE:INLINE")]
        field: Vec<String>,

        /// Text content above the embed (optional)
        #[arg(value_name = "TEXT")]
        content: Option<String>,
    },

    /// Start the hook listener (long-running mode)
    ///
    /// Listens for messages in configured channels and triggers hooks.
    /// Use --foreground to run in foreground (Ctrl+C to stop).
    Listen {
        /// Run in foreground (Ctrl+C to stop)
        ///
        /// Default is to run in background as a daemon.
        #[arg(short, long, default_value = "false")]
        foreground: bool,

        /// Path to hooks configuration file
        ///
        /// Default: ./hooks.yaml
        #[arg(long, value_name = "PATH")]
        hooks_file: Option<PathBuf>,

        /// Path to prompts directory
        ///
        /// Default: ./prompts
        #[arg(short, long, value_name = "PATH")]
        prompts_dir: Option<PathBuf>,

        /// Enable verbose logging
        ///
        /// Shows debug messages
        #[arg(short, long, default_value = "false")]
        verbose: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_help() {
        // Just verify that struct can be created
        let cli = Cli {
            command: None,
            legacy_message: vec![],
        };
        assert!(cli.command.is_none());
        assert!(cli.legacy_message.is_empty());
    }
}
