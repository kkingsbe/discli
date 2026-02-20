//! Input validation functions

use crate::error::{DiscliError, Result};

/// Validate the number of attachments
///
/// Discord allows a maximum of 10 attachments per message.
///
/// # Arguments
///
/// * `count` - Number of attachments to validate
///
/// # Returns
///
/// `Ok(())` if the count is valid
///
/// # Errors
///
/// Returns an error if the count exceeds Discord's limit
pub fn validate_attachment_count(count: usize) -> Result<()> {
    const MAX_ATTACHMENTS: usize = 10;
    if count > MAX_ATTACHMENTS {
        return Err(DiscliError::Validation(format!(
            "Cannot attach more than {} images (got {})",
            MAX_ATTACHMENTS, count
        )));
    }
    Ok(())
}

/// Validate message content length
///
/// Discord allows a maximum of 2000 characters for message content.
///
/// # Arguments
///
/// * `content` - Message content to validate
///
/// # Returns
///
/// `Ok(())` if the content length is valid
///
/// # Errors
///
/// Returns an error if the content exceeds Discord's limit
pub fn validate_content_length(content: &str) -> Result<()> {
    const MAX_LENGTH: usize = 2000;
    if content.len() > MAX_LENGTH {
        return Err(DiscliError::Validation(format!(
            "Message content exceeds Discord's {} character limit (got {})",
            MAX_LENGTH,
            content.len()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_attachment_count_valid() {
        assert!(validate_attachment_count(0).is_ok());
        assert!(validate_attachment_count(5).is_ok());
        assert!(validate_attachment_count(10).is_ok());
    }

    #[test]
    fn test_validate_attachment_count_invalid() {
        assert!(validate_attachment_count(11).is_err());
        assert!(validate_attachment_count(20).is_err());
    }

    #[test]
    fn test_validate_content_length_valid() {
        assert!(validate_content_length("").is_ok());
        assert!(validate_content_length("Hello").is_ok());
        assert!(validate_content_length("a".repeat(2000).as_str()).is_ok());
    }

    #[test]
    fn test_validate_content_length_invalid() {
        assert!(validate_content_length("a".repeat(2001).as_str()).is_err());
        assert!(validate_content_length("a".repeat(5000).as_str()).is_err());
    }
}
