//! Prompt template loader
//!
//! Loads prompt templates from files and manages caching.

use crate::error::{DiscliError, Result};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;

/// A loaded prompt template
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// Template name (filename without extension)
    pub name: String,
    /// Original template content with {{variables}}
    pub content: String,
    /// List of variables used in this template
    pub variables: Vec<String>,
}

impl PromptTemplate {
    /// Load a prompt template from a file
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DiscliError::Io(e))?;
        
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Extract variables
        let variables = extract_variables(&content);
        
        Ok(Self { name, content, variables })
    }
}

/// Extract variable names from template
fn extract_variables(template: &str) -> Vec<String> {
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    let mut vars: Vec<String> = re.captures_iter(template)
        .map(|c| c[1].to_string())
        .collect();
    
    vars.sort();
    vars.dedup();
    vars
}

/// Loader for prompt templates
pub struct PromptLoader {
    /// Base directory for prompts
    prompts_dir: PathBuf,
    /// Cached templates
    cache: HashMap<PathBuf, PromptTemplate>,
}

impl PromptLoader {
    /// Create a new prompt loader
    pub fn new(prompts_dir: PathBuf) -> Self {
        Self {
            prompts_dir,
            cache: HashMap::new(),
        }
    }
    
    /// Load a prompt template
    /// 
    /// Path can be relative (to prompts_dir) or absolute
    pub fn load(&mut self, path: &Path) -> Result<PromptTemplate> {
        // Resolve absolute path
        let absolute_path: PathBuf = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.prompts_dir.join(path)
        };
        
        // Check cache
        if let Some(cached) = self.cache.get(&absolute_path) {
            return Ok(cached.clone());
        }
        
        // Load template
        let template = PromptTemplate::load(&absolute_path)?;
        
        // Cache it
        self.cache.insert(absolute_path, template.clone());
        
        Ok(template)
    }
    
    /// Load all prompt templates from the prompts directory
    pub fn load_all(&mut self) -> Result<Vec<PromptTemplate>> {
        let mut templates = Vec::new();
        
        if !self.prompts_dir.exists() {
            return Ok(templates);
        }
        
        for entry in std::fs::read_dir(&self.prompts_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                match self.load(&path) {
                    Ok(template) => templates.push(template),
                    Err(e) => {
                        eprintln!("Warning: Failed to load prompt {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(templates)
    }
    
    /// Clear the template cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    
    #[test]
    fn test_extract_variables() {
        let template = "Hello {{author_name}}, your message: {{content}}";
        let vars = extract_variables(template);
        
        assert_eq!(vars, vec!["author_name", "content"]);
    }
    
    #[test]
    fn test_prompt_template_load() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        let mut file = std::fs::File::create(&file_path).unwrap();
        file.write_all(b"Hello {{author_name}}!").unwrap();
        
        let template = PromptTemplate::load(&file_path).unwrap();
        
        assert_eq!(template.name, "test");
        assert_eq!(template.content, "Hello {{author_name}}!");
        assert_eq!(template.variables, vec!["author_name"]);
    }
}
