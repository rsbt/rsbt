/*!
# rsbt-draf description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-draf = "0.1"
```

*/

mod commands;
mod error;
mod properties;
mod result;
mod sync;
mod tasks;
mod tokio;
mod transport;

pub use tasks::App as RsbtApp;

pub use crate::tokio::TokioApp as RsbtTokioApp;
pub use error::RsbtError;
pub use properties::RsbtAppProperties;
pub use result::RsbtResult;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
