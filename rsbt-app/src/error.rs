/// RSBT application level error.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Runtime error: {0}")]
    Runtime(#[from] rsbt_rt::RuntimeError),
    #[error("Runtime could not be init: {0}")]
    RuntimeInit(std::io::Error),
    #[error("unknown error")]
    Unknown,
}
