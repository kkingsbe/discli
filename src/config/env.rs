//! Environment configuration for discli

use crate::error::{DiscliError, Result};
use std::env;
use std::path::PathBuf;

/// Configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Discord bot token
    pub discord_token: String,
    /// Discord channel ID to send messages to
    pub channel_id: String,
    
    // Hook system configuration
    /// Whether hook system is enabled
    pub hook_enabled: bool,
    /// Path to hooks.yaml file
    pub hooks_file: PathBuf,
    /// Directory for prompt templates
    pub prompts_dir: PathBuf,
    /// Logging level
    pub log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// This function attempts to load configuration from the following sources:
    /// 1. First, it tries to load a `discli.env` file if it exists
    /// 2. Then, it reads `DISCORD_TOKEN` and `DISCORD_CHANNEL_ID` from environment variables
    ///
    /// # Returns
    ///
    /// Returns a `Config` struct containing the loaded configuration
    ///
    /// # Errors
    ///
    /// Returns an error if either `DISCORD_TOKEN` or `DISCORD_CHANNEL_ID` is not set
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use discli::config::Config;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::load()?;
    /// println!("Channel ID: {}", config.channel_id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn load() -> Result<Self> {
        // Try to load .env file (ignore errors if file doesn't exist)
        dotenv::from_filename("discli.env").ok();

        // Load required environment variables
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| DiscliError::Config("DISCORD_TOKEN not set".into()))?;

        let channel_id = env::var("DISCORD_CHANNEL_ID")
            .map_err(|_| DiscliError::Config("DISCORD_CHANNEL_ID not set".into()))?;

        // Hook system configuration (with defaults)
        let hook_enabled = env::var("HOOK_ENABLED")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
            
        let hooks_file = env::var("HOOKS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./hooks.yaml"));
            
        let prompts_dir = env::var("PROMPTS_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./prompts"));
            
        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            discord_token,
            channel_id,
            hook_enabled,
            hooks_file,
            prompts_dir,
            log_level,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires environment variables to be set
    fn test_load_config() {
        // This test requires actual environment variables to be set
        // Run with: DISCORD_TOKEN=test DISCORD_CHANNEL_ID=123 cargo test
        let result = Config::load();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.discord_token, "test");
        assert_eq!(config.channel_id, "123");
        assert!(!config.hook_enabled);
        assert_eq!(config.hooks_file, PathBuf::from("./hooks.yaml"));
        assert_eq!(config.prompts_dir, PathBuf::from("./prompts"));
        assert_eq!(config.log_level, "info");
    }
}
