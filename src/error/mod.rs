use std::{error::Error, fmt};
#[derive(Debug)]
pub struct RsbtError;

impl fmt::Display for RsbtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RsbtError is here!")
    }
}

impl Error for RsbtError {}
