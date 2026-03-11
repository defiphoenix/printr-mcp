use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("failed to execute order: {0}")]
    Failed(String),
}
