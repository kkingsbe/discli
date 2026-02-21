//! Hook configuration types and loading

use crate::error::{DiscliError, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main hooks configuration container
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HooksConfig {
    /// Configuration file version
    pub version: String,
    /// Global settings
    #[serde(default)]
    pub settings: Settings,
    /// Hook definitions
    pub hooks: Vec<HookConfig>,
    /// Directory for prompt templates
    #[serde(default = "default_prompts_dir")]
    pub prompts_dir: PathBuf,
}

fn default_prompts_dir() -> PathBuf {
    PathBuf::from("./prompts")
}

/// Global settings for hook system
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    /// Error handling strategy
    #[serde(default)]
    pub on_error: ErrorStrategy,
    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            on_error: ErrorStrategy::Log,
            rate_limit: RateLimitConfig::default(),
        }
    }
}

/// Error handling strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorStrategy {
    Log,
    Ignore,
    Notify,
}

impl Default for ErrorStrategy {
    fn default() -> Self {
        ErrorStrategy::Log
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitConfig {
    /// Maximum triggers per user
    #[serde(default = "default_rate_limit")]
    pub per_user: u32,
    /// Maximum triggers per channel
    #[serde(default = "default_rate_limit")]
    pub per_channel: u32,
    /// Time window in seconds
    #[serde(default = "default_window")]
    pub window_seconds: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            per_user: 5,
            per_channel: 10,
            window_seconds: 60,
        }
    }
}

fn default_rate_limit() -> u32 { 5 }
fn default_window() -> u64 { 60 }

/// Individual hook configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookConfig {
    /// Unique identifier for this hook
    pub id: String,
    /// Human-readable name
    #[serde(default)]
    pub name: String,
    /// Whether this hook is active
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Channel IDs to listen on (as strings for flexibility)
    pub channels: Vec<String>,
    /// Trigger configuration
    pub trigger: TriggerConfig,
    /// Path to prompt file (relative to prompts_dir or absolute)
    pub prompt_file: PathBuf,
    /// Optional filter for specific users/roles
    #[serde(default)]
    pub filter: Option<FilterConfig>,
    /// Action to take when hook triggers
    pub action: HookAction,
    /// Processing configuration
    #[serde(default)]
    pub processing: ProcessingConfig,
}

fn default_enabled() -> bool { true }

/// Trigger configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TriggerConfig {
    /// Trigger on any message
    Any,
    /// Trigger when message starts with prefix
    Prefix { prefix: String },
    /// Trigger when message contains substring
    Contains { substring: String },
    /// Trigger when regex matches
    Regex { pattern: String },
    /// Trigger when bot is mentioned
    Mention,
}

/// Filter for specific users/roles
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterConfig {
    /// Only trigger for these user IDs
    #[serde(default)]
    pub users: Vec<String>,
    /// Only trigger for users with these role IDs
    #[serde(default)]
    pub roles: Vec<String>,
}

/// Action to perform when hook triggers
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HookAction {
    /// Reply in the channel
    Reply,
    /// Send DM to the user
    SendDm,
    /// Forward to another channel
    Forward { channel_id: String },
    /// Send to webhook URL
    Webhook { url: String },
}

/// Processing configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessingConfig {
    /// Timeout for processing in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    /// Processor type: "command" or "http"
    #[serde(rename = "processor_type", default)]
    pub processor_type: String,
    /// Command to execute (for command processor)
    #[serde(default)]
    pub cmd: Vec<String>,
    /// HTTP URL (for http processor)
    #[serde(default)]
    pub url: String,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            processor_type: "command".to_string(),
            cmd: vec![],
            url: String::new(),
        }
    }
}

fn default_timeout() -> u64 { 30 }

/// Processor backend
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Processor {
    /// Execute a command with prompt as stdin
    Command {
        /// Command and arguments
        cmd: Vec<String>,
    },
    /// Call an HTTP endpoint
    Http {
        /// POST endpoint URL
        url: String,
    },
}

impl Processor {
    pub fn from_yaml(type_str: &str, map: &serde_yaml::Value) -> Self {
        eprintln!("[DEBUG] Processor::from_yaml type={}, map={:?}", type_str, map);
        match type_str {
            "command" => {
                if let Some(cmd) = map.get("cmd") {
                    if let Ok(cmd_vec) = serde_yaml::from_value::<Vec<String>>(cmd.clone()) {
                        return Processor::Command { cmd: cmd_vec };
                    }
                }
                Processor::Command { cmd: vec![] }
            }
            "http" => {
                if let Some(url) = map.get("url") {
                    if let Ok(url_str) = serde_yaml::from_value::<String>(url.clone()) {
                        return Processor::Http { url: url_str };
                    }
                }
                Processor::Http { url: String::new() }
            }
            _ => Processor::Command { cmd: vec![] },
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Processor::Command { cmd: vec![] }
    }
}

impl HooksConfig {
    /// Load hooks configuration from a YAML file
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DiscliError::Config(format!("Failed to read hooks file: {}", e)))?;
        
        let config: HooksConfig = serde_yaml::from_str(&content)
            .map_err(|e| DiscliError::Config(format!("Failed to parse hooks.yaml: {}", e)))?;
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }
    
    /// Get only enabled hooks
    pub fn enabled_hooks(&self) -> Vec<&HookConfig> {
        self.hooks.iter().filter(|h| h.enabled).collect()
    }
    
    fn validate(&self) -> std::result::Result<(), DiscliError> {
        if self.hooks.is_empty() {
            return Err(DiscliError::Config("No hooks defined".into()));
        }
        
        for hook in &self.hooks {
            if hook.channels.is_empty() {
                return Err(DiscliError::Config(format!(
                    "Hook '{}' has no channels defined", hook.id
                )));
            }
        }
        
        Ok(())
    }
}

impl HookConfig {
    /// Compile regex patterns if needed
    pub fn compile(&self) -> Result<CompiledHookConfig> {
        let trigger = match &self.trigger {
            TriggerConfig::Regex { pattern } => {
                CompiledTrigger::Regex(Regex::new(pattern)
                    .map_err(|e| DiscliError::Config(format!("Invalid regex: {}", e)))?)
            }
            other => CompiledTrigger::from(other.clone()),
        };
        
        Ok(CompiledHookConfig {
            id: self.id.clone(),
            name: self.name.clone(),
            channels: self.channels.clone(),
            trigger,
            prompt_file: self.prompt_file.clone(),
            filter: self.filter.clone(),
            action: self.action.clone(),
            processing: self.processing.clone(),
        })
    }
}

/// Compiled hook config (with regex pre-compiled)
pub struct CompiledHookConfig {
    pub id: String,
    pub name: String,
    pub channels: Vec<String>,
    pub trigger: CompiledTrigger,
    pub prompt_file: PathBuf,
    pub filter: Option<FilterConfig>,
    pub action: HookAction,
    pub processing: ProcessingConfig,
}

/// Trigger with compiled regex
#[derive(Debug)]
pub enum CompiledTrigger {
    Any,
    Prefix(String),
    Contains(String),
    Regex(Regex),
    Mention,
}

impl From<TriggerConfig> for CompiledTrigger {
    fn from(config: TriggerConfig) -> Self {
        match config {
            TriggerConfig::Any => CompiledTrigger::Any,
            TriggerConfig::Prefix { prefix } => CompiledTrigger::Prefix(prefix),
            TriggerConfig::Contains { substring } => CompiledTrigger::Contains(substring),
            TriggerConfig::Regex { pattern } => {
                // Note: This will panic if regex is invalid - use compile() instead
                CompiledTrigger::Regex(Regex::new(&pattern).unwrap())
            }
            TriggerConfig::Mention => CompiledTrigger::Mention,
        }
    }
}
