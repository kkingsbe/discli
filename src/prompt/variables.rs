//! Variable extraction and substitution for prompt templates
//!
//! Provides MessageVariables for extracting data from Discord messages
//! and substitute_variables for template variable replacement.

use regex::Regex;
use std::collections::HashMap;

/// Variables extracted from a Discord message
#[derive(Debug, Clone)]
pub struct MessageVariables {
    /// Full message content
    pub content: String,
    /// User's Discord ID
    pub author_id: String,
    /// User's username (with discriminator if applicable)
    pub author_name: String,
    /// Channel ID where message was sent
    pub channel_id: String,
    /// Message ID
    pub message_id: String,
    /// Message timestamp (ISO 8601)
    pub timestamp: String,
    /// List of attachment filenames
    pub attachments: Vec<String>,
    /// Number of embeds in message
    pub embed_count: usize,
}

impl MessageVariables {
    /// Create from a twilight MessageCreate event
    pub fn from_message(msg: &twilight_model::gateway::payload::incoming::MessageCreate) -> Self {
        let attachments: Vec<String> = msg.0
            .attachments
            .iter()
            .map(|a| a.filename.clone())
            .collect();
            
        Self {
            content: msg.0.content.clone(),
            author_id: msg.0.author.id.to_string(),
            author_name: msg.0.author.name.clone(),
            channel_id: msg.0.channel_id.to_string(),
            message_id: msg.0.id.to_string(),
            timestamp: msg.0.timestamp.iso_8601().to_string(),
            attachments,
            embed_count: msg.0.embeds.len(),
        }
    }
    
    /// Get all variables as a map for substitution
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("content".to_string(), self.content.clone());
        map.insert("author_id".to_string(), self.author_id.clone());
        map.insert("author_name".to_string(), self.author_name.clone());
        map.insert("channel_id".to_string(), self.channel_id.clone());
        map.insert("message_id".to_string(), self.message_id.clone());
        map.insert("timestamp".to_string(), self.timestamp.clone());
        map.insert("attachments".to_string(), self.attachments.join(", "));
        map.insert("embed_count".to_string(), self.embed_count.to_string());
        map
    }
}

/// Substitute {{variable}} placeholders in template
pub fn substitute_variables(template: &str, vars: &MessageVariables) -> String {
    let var_map = vars.to_map();
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    
    re.replace_all(template, |caps: &regex::Captures| {
        let var_name = &caps[1];
        var_map.get(var_name).cloned().unwrap_or_else(|| format!("{{{{{}}}}}", var_name))
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_substitute_variables() {
        let vars = MessageVariables {
            content: "Hello world".to_string(),
            author_id: "123".to_string(),
            author_name: "testuser".to_string(),
            channel_id: "456".to_string(),
            message_id: "789".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            attachments: vec!["image.png".to_string()],
            embed_count: 0,
        };
        
        let template = "User {{author_name}} said: {{content}}";
        let result = substitute_variables(template, &vars);
        
        assert_eq!(result, "User testuser said: Hello world");
    }
    
    #[test]
    fn test_unknown_variable_unchanged() {
        let vars = MessageVariables {
            content: "test".to_string(),
            author_id: "1".to_string(),
            author_name: "user".to_string(),
            channel_id: "2".to_string(),
            message_id: "3".to_string(),
            timestamp: "t".to_string(),
            attachments: vec![],
            embed_count: 0,
        };
        
        let template = "Field: {{unknown_field}}";
        let result = substitute_variables(template, &vars);
        
        assert_eq!(result, "Field: {{unknown_field}}");
    }
}
