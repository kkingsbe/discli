//! Error types and handling for discli

use thiserror::Error;

/// Main error type for discli operations
#[derive(Debug, Error)]
pub enum DiscliError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Discord API errors
    #[error("Discord API error: {0}")]
    DiscordApi(String),

    /// Network errors from reqwest
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// I/O errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Attachment-related errors
    #[error("Attachment error: {0}")]
    Attachment(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// MIME type errors
    #[error("MIME type error: {0}")]
    Mime(String),

    /// Gateway connection errors
    #[error("Gateway error: {0}")]
    Gateway(String),

    /// WebSocket errors
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, DiscliError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DiscliError::Config("Test error".to_string());
        assert_eq!(err.to_string(), "Configuration error: Test error");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let discli_err: DiscliError = io_err.into();
        assert!(matches!(discli_err, DiscliError::Io(_)));
    }
}
