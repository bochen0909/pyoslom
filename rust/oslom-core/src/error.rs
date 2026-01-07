use thiserror::Error;

pub type Result<T> = std::result::Result<T, OslomError>;

#[derive(Error, Debug)]
pub enum OslomError {
    #[error("Invalid network structure: {0}")]
    InvalidNetwork(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Algorithm convergence failed: {0}")]
    ConvergenceFailed(String),
    
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Numerical error: {0}")]
    Numerical(String),
}