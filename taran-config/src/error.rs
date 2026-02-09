use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Invalid scenario: {0}")]
    InvalidScenario(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid duration format: {0}")]
    InvalidDuration(String),
}

pub type Result<T> = std::result::Result<T, ConfigError>;
