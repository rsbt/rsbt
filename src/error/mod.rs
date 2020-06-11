use thiserror::Error;

#[derive(Error, Debug)]
pub enum RsbtError {
    #[error("handshake failure {0}")]
    Handshake(String),
}
