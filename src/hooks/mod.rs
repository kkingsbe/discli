//! Hook system for processing Discord messages
//! 
//! This module provides:
//! - Hook configuration loading from YAML
//! - Trigger matching (prefix, regex, mention)
//! - Hook execution with prompt processing

pub mod config;
pub mod trigger;
pub mod executor;

pub use config::{HookConfig, HooksConfig, TriggerConfig, FilterConfig, HookAction, Processor, CompiledHookConfig, CompiledTrigger};
pub use trigger::{TriggerMatcher, should_trigger, matches_filter, matches_channels};
pub use executor::{HookExecutor, HookResult, RateLimiter};
pub use crate::processing::{CommandProcessor, HttpProcessor};
