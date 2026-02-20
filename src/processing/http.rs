use crate::error::{DiscliError, Result};
use reqwest::Client;
use serde_json::json;

/// Process messages via HTTP webhook
pub struct HttpProcessor {
    client: Client,
    timeout_secs: u64,
}

impl HttpProcessor {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            client: Client::new(),
            timeout_secs,
        }
    }
    
    /// Send prompt to HTTP endpoint
    /// 
    /// The URL should accept POST requests with JSON body:
    /// {
    ///   "prompt": "...",
    ///   "metadata": { ... }
    /// }
    pub async fn execute(&self, url: &str, prompt: &str, metadata: Option<serde_json::Value>) -> Result<String> {
        let body = json!({
            "prompt": prompt,
            "metadata": metadata.unwrap_or(json!({}))
        });
        
        let response = self.client
            .post(url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(self.timeout_secs))
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(DiscliError::DiscordApi(format!(
                "HTTP webhook failed: {} - {}",
                status, text
            )));
        }
        
        let text = response.text().await
            .map_err(|e| DiscliError::Network(e))?;
        
        Ok(text)
    }
}
