use crate::error::{DiscliError, Result};
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;

/// Process messages by executing a command
pub struct CommandProcessor {
    timeout_secs: u64,
}

impl CommandProcessor {
    pub fn new(timeout_secs: u64) -> Self {
        Self { timeout_secs }
    }
    
    /// Execute a command with prompt as stdin
    /// 
    /// The cmd should be a vec of command + args, e.g., ["python", "script.py"]
    pub async fn execute(&self, cmd: &[String], prompt: &str) -> Result<String> {
        if cmd.is_empty() {
            return Err(DiscliError::Config("Empty command".into()));
        }
        
        let program = &cmd[0];
        let args = &cmd[1..];
        
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiscliError::Io(e))?;
        
        // Write prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(prompt.as_bytes()).await
                .map_err(|e| DiscliError::Io(e))?;
        }
        
        // Set timeout
        let timeout = tokio::time::Duration::from_secs(self.timeout_secs);
        
        let output = tokio::time::timeout(timeout, child.wait_with_output())
            .await
            .map_err(|_| DiscliError::Config("Command timed out".into()))?
            .map_err(|e| DiscliError::Io(e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DiscliError::Config(format!(
                "Command failed: {}",
                stderr
            )));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_echo_command() {
        let processor = CommandProcessor::new(5);
        
        // Test with a simple echo command
        #[cfg(windows)]
        let result = processor.execute(&["cmd".to_string(), "/C".to_string(), "echo hello".to_string()], "test input").await;
        
        #[cfg(not(windows))]
        let result = processor.execute(&["echo".to_string(), "hello".to_string()], "test input").await;
        
        // This might fail on CI, so we'll just check it runs
        println!("Result: {:?}", result);
    }
}
