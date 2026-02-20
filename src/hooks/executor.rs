//! Hook executor for processing and responding to messages
//!
//! This module handles executing hook actions and processing prompts.

use crate::config::Config;
use crate::discord::DiscordClient;
use crate::discord::types::DiscordMessage;
use crate::hooks::config::{CompiledHookConfig, HookAction, ProcessingConfig};
use crate::processing::{CommandProcessor, HttpProcessor};
use crate::prompt::variables::MessageVariables;
use crate::prompt::registry::PromptRegistry;
use crate::error::Result;
use tokio::sync::RwLock;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use twilight_model::gateway::payload::incoming::MessageCreate;

/// Rate limiter for hooks
pub struct RateLimiter {
    per_user: u32,
    per_channel: u32,
    window: Duration,
    user_history: RwLock<VecDeque<(String, Instant)>>,
    channel_history: RwLock<VecDeque<(String, Instant)>>,
}

impl RateLimiter {
    pub fn new(per_user: u32, per_channel: u32, window_secs: u64) -> Self {
        Self {
            per_user,
            per_channel,
            window: Duration::from_secs(window_secs),
            user_history: RwLock::new(VecDeque::new()),
            channel_history: RwLock::new(VecDeque::new()),
        }
    }
    
    /// Check if user is rate limited
    pub async fn check_user(&self, user_id: &str) -> bool {
        let mut history = self.user_history.write().await;
        let now = Instant::now();
        
        // Remove old entries
        while history.front().map(|(_, t)| now.duration_since(*t) > self.window).unwrap_or(false) {
            history.pop_front();
        }
        
        // Count recent from this user
        let count = history.iter()
            .filter(|(id, _)| id == user_id)
            .count();
        
        if count >= self.per_user as usize {
            return false; // Rate limited
        }
        
        history.push_back((user_id.to_string(), now));
        true
    }
    
    /// Check if channel is rate limited
    pub async fn check_channel(&self, channel_id: &str) -> bool {
        let mut history = self.channel_history.write().await;
        let now = Instant::now();
        
        // Remove old entries
        while history.front().map(|(_, t)| now.duration_since(*t) > self.window).unwrap_or(false) {
            history.pop_front();
        }
        
        // Count recent from this channel
        let count = history.iter()
            .filter(|(id, _)| id == channel_id)
            .count();
        
        if count >= self.per_channel as usize {
            return false; // Rate limited
        }
        
        history.push_back((channel_id.to_string(), now));
        true
    }
}

/// Result of hook execution
#[derive(Debug)]
pub struct HookResult {
    /// Whether the hook was executed
    pub executed: bool,
    /// The response message (if any)
    pub response: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
}

/// Hook executor
pub struct HookExecutor {
    config: Config,
    prompt_registry: PromptRegistry,
    rate_limiter: RateLimiter,
}

impl HookExecutor {
    pub fn new(config: Config) -> Self {
        let prompts_dir = config.prompts_dir.clone();
        let rate_limiter = RateLimiter::new(
            5, // per_user
            10, // per_channel
            60, // window
        );
        
        Self {
            config,
            prompt_registry: PromptRegistry::new(prompts_dir),
            rate_limiter,
        }
    }
    
    /// Execute a hook for a message
    pub async fn execute(
        &mut self,
        hook: &CompiledHookConfig,
        message: &MessageCreate,
    ) -> Result<HookResult> {
        // Check rate limits
        let user_id = message.0.author.id.to_string();
        let channel_id = message.0.channel_id.to_string();
        
        if !self.rate_limiter.check_user(&user_id).await {
            return Ok(HookResult {
                executed: false,
                response: None,
                error: Some("Rate limited (user)".to_string()),
            });
        }
        
        if !self.rate_limiter.check_channel(&channel_id).await {
            return Ok(HookResult {
                executed: false,
                response: None,
                error: Some("Rate limited (channel)".to_string()),
            });
        }
        
        // Extract variables from message
        let vars = MessageVariables::from_message(message);
        
        // Render prompt with variables
        let prompt = match self.prompt_registry.render(&hook.prompt_file, &vars) {
            Ok(p) => p,
            Err(e) => {
                return Ok(HookResult {
                    executed: false,
                    response: None,
                    error: Some(format!("Failed to load prompt: {}", e)),
                });
            }
        };
        
        // Execute processor (placeholder - Phase 6)
        let response = self.execute_processor(&hook.processing, &prompt).await?;
        
        // Send response based on action
        self.send_response(&hook.action, &response, message).await?;
        
        Ok(HookResult {
            executed: true,
            response: Some(response),
            error: None,
        })
    }
    
    /// Execute the processor
    async fn execute_processor(
        &self,
        processing: &ProcessingConfig,
        prompt: &str,
    ) -> Result<String> {
        match processing.processor_type.as_str() {
            "command" => {
                if processing.cmd.is_empty() {
                    return Err(crate::error::DiscliError::Config("No command configured".into()));
                }
                let processor = CommandProcessor::new(processing.timeout_seconds);
                processor.execute(&processing.cmd, prompt).await
            }
            "http" => {
                if processing.url.is_empty() {
                    return Err(crate::error::DiscliError::Config("No URL configured".into()));
                }
                let processor = HttpProcessor::new(processing.timeout_seconds);
                processor.execute(&processing.url, prompt, None).await
            }
            _ => Err(crate::error::DiscliError::Config(
                format!("Unknown processor type: {}", processing.processor_type)
            ).into())
        }
    }
    
    /// Send response based on action
    async fn send_response(
        &self,
        action: &HookAction,
        response: &str,
        message: &MessageCreate,
    ) -> Result<()> {
        match action {
            HookAction::Reply => {
                // Create Discord client and send message to channel
                let client = DiscordClient::new(self.config.discord_token.clone());
                
                let msg = DiscordMessage::Simple {
                    content: response.to_string(),
                };
                
                let channel_id = message.0.channel_id.to_string();
                client.send_message(&channel_id, &msg).await?;
                
                println!("[HOOK] Replied to channel {}: {}", channel_id, &response[..response.len().min(50)]);
            }
            HookAction::SendDm => {
                // Would need to create DM channel first via Discord API
                // For now, just log
                println!("[HOOK] Would DM user: {}", response);
            }
            HookAction::Forward { channel_id } => {
                let client = DiscordClient::new(self.config.discord_token.clone());
                
                let msg = DiscordMessage::Simple {
                    content: response.to_string(),
                };
                
                client.send_message(channel_id, &msg).await?;
                
                println!("[HOOK] Forwarded to {}: {}", channel_id, &response[..response.len().min(50)]);
            }
            HookAction::Webhook { url } => {
                let client = reqwest::Client::new();
                
                let _ = client.post(url)
                    .json(&serde_json::json!({
                        "content": response,
                    }))
                    .send()
                    .await;
                
                println!("[HOOK] Sent to webhook: {}", &response[..response.len().min(50)]);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_rate_limiter_user() {
        let limiter = RateLimiter::new(2, 10, 60);
        
        // First two should pass
        assert!(limiter.check_user("user1").await);
        assert!(limiter.check_user("user1").await);
        
        // Third should be rate limited
        assert!(!limiter.check_user("user1").await);
        
        // Different user should pass
        assert!(limiter.check_user("user2").await);
    }
    
    #[tokio::test]
    async fn test_rate_limiter_channel() {
        let limiter = RateLimiter::new(5, 2, 60);
        
        // First two should pass
        assert!(limiter.check_channel("chan1").await);
        assert!(limiter.check_channel("chan1").await);
        
        // Third should be rate limited
        assert!(!limiter.check_channel("chan1").await);
        
        // Different channel should pass
        assert!(limiter.check_channel("chan2").await);
    }
    
    #[test]
    fn test_hook_result_default() {
        let result = HookResult {
            executed: false,
            response: None,
            error: None,
        };
        
        assert!(!result.executed);
        assert!(result.response.is_none());
        assert!(result.error.is_none());
    }
    
    #[test]
    fn test_hook_action_reply() {
        let action = HookAction::Reply;
        match action {
            HookAction::Reply => assert!(true),
            _ => panic!("Expected Reply variant"),
        }
    }
    
    #[test]
    fn test_hook_action_send_dm() {
        let action = HookAction::SendDm;
        match action {
            HookAction::SendDm => assert!(true),
            _ => panic!("Expected SendDm variant"),
        }
    }
    
    #[test]
    fn test_hook_action_forward() {
        let action = HookAction::Forward { channel_id: "123".to_string() };
        match action {
            HookAction::Forward { channel_id } => assert_eq!(channel_id, "123"),
            _ => panic!("Expected Forward variant"),
        }
    }
    
    #[test]
    fn test_hook_action_webhook() {
        let action = HookAction::Webhook { url: "https://example.com".to_string() };
        match action {
            HookAction::Webhook { url } => assert_eq!(url, "https://example.com"),
            _ => panic!("Expected Webhook variant"),
        }
    }
}
