/// RSBT application level error.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Runtime could not be init: {0}")]
    RuntimeInit(std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("unknown error")]
    Unknown,
}
