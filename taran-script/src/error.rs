use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("Script execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Script parse error: {0}")]
    ParseError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, ScriptError>;
