/// RSBT application level error.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unknown error")]
    Unknown,
}
