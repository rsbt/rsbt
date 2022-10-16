/// RSBT application level error.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Runtime could not be init: {0}")]
    Runtime(std::io::Error),
    #[error("unknown error")]
    Unknown,
}
