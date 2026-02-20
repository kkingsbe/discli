//! Prompt template system for hooks
//! 
//! Provides:
//! - Loading prompt templates from files
//! - Variable substitution with message data
//! - Template validation

pub mod loader;
pub mod variables;
pub mod registry;

pub use loader::PromptLoader;
pub use loader::PromptTemplate;
pub use variables::{MessageVariables, substitute_variables};
pub use registry::PromptRegistry;
