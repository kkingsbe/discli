//! Prompt registry for managing available templates
//!
//! Provides a centralized way to access and render prompt templates.

use super::loader::PromptLoader;
use super::variables::MessageVariables;
use crate::error::Result;
use std::path::PathBuf;

/// Registry of available prompt templates
pub struct PromptRegistry {
    loader: PromptLoader,
}

impl PromptRegistry {
    /// Create a new registry
    pub fn new(prompts_dir: PathBuf) -> Self {
        Self {
            loader: PromptLoader::new(prompts_dir),
        }
    }
    
    /// Get a prompt template by path
    pub fn get(&mut self, path: &PathBuf) -> Result<super::loader::PromptTemplate> {
        self.loader.load(path)
    }
    
    /// Get all available prompts
    pub fn all(&mut self) -> Result<Vec<super::loader::PromptTemplate>> {
        self.loader.load_all()
    }
    
    /// Load and substitute a prompt with variables
    pub fn render(
        &mut self, 
        path: &PathBuf, 
        vars: &MessageVariables
    ) -> Result<String> {
        let template = self.get(path)?;
        Ok(super::variables::substitute_variables(&template.content, vars))
    }
}
