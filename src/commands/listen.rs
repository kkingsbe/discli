//! Listen command implementation - starts the hook listener

use crate::config::Config;
use crate::discord::DiscordGateway;
use crate::hooks::config::{CompiledHookConfig, HooksConfig};
use crate::hooks::executor::HookExecutor;
use crate::hooks::trigger::should_trigger;
use crate::error::{DiscliError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_model::gateway::payload::incoming::MessageCreate;

/// Execute the listen command - starts the hook listener
pub async fn execute(
    config: &Config,
    hooks_file: Option<std::path::PathBuf>,
    prompts_dir: Option<std::path::PathBuf>,
    verbose: bool,
) -> Result<()> {
    // Load hook configuration
    let hooks_path = hooks_file.unwrap_or_else(|| config.hooks_file.clone());
    
    if !hooks_path.exists() {
        return Err(DiscliError::Config(format!(
            "Hooks file not found: {} (use --hooks-file or create hooks.yaml)",
            hooks_path.display()
        )));
    }
    
    let hooks_config = HooksConfig::load(&hooks_path)?;
    
    // Override prompts_dir if provided
    let prompts_dir = prompts_dir.unwrap_or_else(|| config.prompts_dir.clone());
    
    if verbose {
        println!("Loaded {} hooks from {}", hooks_config.hooks.len(), hooks_path.display());
        println!("Prompts directory: {}", prompts_dir.display());
    }
    
    // Compile hooks
    let mut compiled_hooks: Vec<CompiledHookConfig> = Vec::new();
    for hook in hooks_config.enabled_hooks() {
        match hook.compile() {
            Ok(compiled) => {
                if verbose {
                    println!("Compiled hook: {} ({:?})", compiled.id, compiled.trigger);
                }
                compiled_hooks.push(compiled);
            }
            Err(e) => {
                eprintln!("Warning: Failed to compile hook {}: {}", hook.id, e);
            }
        }
    }
    
    if compiled_hooks.is_empty() {
        return Err(DiscliError::Config("No valid hooks to execute".into()));
    }
    
    println!("Starting Discord gateway...");
    println!("Press Ctrl+C to stop");
    
    // Create gateway
    let gateway = DiscordGateway::new(config.discord_token.clone());
    
    // Create hook executor
    let mut executor_config = config.clone();
    executor_config.prompts_dir = prompts_dir;
    let executor = Arc::new(RwLock::new(HookExecutor::new(executor_config)));
    
    // Shared compiled hooks
    let hooks = Arc::new(RwLock::new(compiled_hooks));
    
    // Start listening
    gateway.listen(move |event: MessageCreate| {
        let hooks = Arc::clone(&hooks);
        let executor = Arc::clone(&executor);
        
        tokio::spawn(async move {
            // Get current hooks
            let hooks = hooks.read().await;
            
            // Check each hook
            for hook in hooks.iter() {
                if should_trigger(hook, &event) {
                    if verbose {
                        println!("Triggering hook: {}", hook.id);
                    }
                    
                    let mut executor = executor.write().await;
                    match executor.execute(hook, &event).await {
                        Ok(result) => {
                            if verbose {
                                if let Some(response) = result.response {
                                    println!("Hook {} executed: {}", hook.id, response);
                                }
                                if let Some(error) = result.error {
                                    eprintln!("Hook {} error: {}", hook.id, error);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Hook {} execution failed: {}", hook.id, e);
                        }
                    }
                }
            }
        });
    }).await?;
    
    Ok(())
}
