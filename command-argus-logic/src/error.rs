use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum CommandArgusError {
    #[error("Command not found: {0}")]
    NotFound(Uuid),
    
    #[error("Command with name '{0}' already exists")]
    DuplicateName(String),
    
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, CommandArgusError>;