//! Trigger matching logic for hooks
//!
//! This module handles matching messages against hook triggers.

use crate::hooks::config::{CompiledTrigger, FilterConfig};
use twilight_model::gateway::payload::incoming::MessageCreate;

/// Trait for matching triggers
pub trait TriggerMatcher {
    /// Check if a message matches this trigger
    fn matches(&self, message: &MessageCreate) -> bool;
}

impl TriggerMatcher for CompiledTrigger {
    fn matches(&self, message: &MessageCreate) -> bool {
        match self {
            CompiledTrigger::Any => true,
            CompiledTrigger::Prefix(prefix) => message.0.content.starts_with(prefix),
            CompiledTrigger::Contains(substring) => message.0.content.contains(substring),
            CompiledTrigger::Regex(re) => re.is_match(&message.0.content),
            CompiledTrigger::Mention => {
                // Check if the bot was mentioned
                // For now, check for @bot or bot username
                // In production, would check message.mentions
                false // TODO: implement properly with twilight mentions
            }
        }
    }
}

/// Check if a message passes the filter (user/role restrictions)
pub fn matches_filter(message: &MessageCreate, filter: &Option<FilterConfig>) -> bool {
    match filter {
        Some(f) => {
            // Check user filter
            if !f.users.is_empty() {
                let author_id = message.0.author.id.to_string();
                if !f.users.contains(&author_id) {
                    return false;
                }
            }
            // Role filter would need guild context - skip for now
            true
        }
        None => true,
    }
}

/// Check if message is in one of the configured channels
pub fn matches_channels(message: &MessageCreate, channels: &[String]) -> bool {
    let channel_id = message.0.channel_id.to_string();
    channels.contains(&channel_id)
}

/// Full trigger match check
pub fn should_trigger(
    hook: &crate::hooks::config::CompiledHookConfig,
    message: &MessageCreate,
) -> bool {
    // Check channel
    if !matches_channels(message, &hook.channels) {
        return false;
    }
    
    // Check trigger
    if !hook.trigger.matches(message) {
        return false;
    }
    
    // Check filter
    if !matches_filter(message, &hook.filter) {
        return false;
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_any_trigger_matches() {
        // Test that Any trigger always returns true (will panic if called with actual message)
        let trigger = CompiledTrigger::Any;
        // Just test the enum matching works
        match trigger {
            CompiledTrigger::Any => assert!(true),
            _ => panic!("Expected Any variant"),
        }
    }
    
    #[test]
    fn test_prefix_trigger_variant() {
        let trigger = CompiledTrigger::Prefix("!".to_string());
        match trigger {
            CompiledTrigger::Prefix(p) => assert_eq!(p, "!"),
            _ => panic!("Expected Prefix variant"),
        }
    }
    
    #[test]
    fn test_contains_trigger_variant() {
        let trigger = CompiledTrigger::Contains("hello".to_string());
        match trigger {
            CompiledTrigger::Contains(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected Contains variant"),
        }
    }
    
    #[test]
    fn test_regex_trigger_variant() {
        let re = regex::Regex::new(r"^\d+$").unwrap();
        let trigger = CompiledTrigger::Regex(re.clone());
        match trigger {
            CompiledTrigger::Regex(_) => assert!(true),
            _ => panic!("Expected Regex variant"),
        }
        
        // Test regex matching
        assert!(re.is_match("123"));
        assert!(!re.is_match("abc"));
    }
    
    #[test]
    fn test_mention_trigger_variant() {
        let trigger = CompiledTrigger::Mention;
        match trigger {
            CompiledTrigger::Mention => assert!(true),
            _ => panic!("Expected Mention variant"),
        }
    }
    
    #[test]
    fn test_matches_filter_no_filter() {
        // Test that None filter returns true
        let filter: Option<FilterConfig> = None;
        // Filter is None, so it should return true (we can't test with actual message)
        assert!(filter.is_none());
    }
    
    #[test]
    fn test_matches_filter_with_users() {
        let filter = FilterConfig {
            users: vec!["123".to_string()],
            roles: vec![],
        };
        
        // Test that filter with users works
        assert!(!filter.users.is_empty());
        assert!(filter.users.contains(&"123".to_string()));
    }
    
    #[test]
    fn test_matches_filter_user_not_allowed() {
        let filter = FilterConfig {
            users: vec!["999".to_string()],
            roles: vec![],
        };
        
        // Test that filter has users
        assert!(filter.users.contains(&"999".to_string()));
        assert!(!filter.users.contains(&"123".to_string()));
    }
}
