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
mod result;
mod sync;
mod tasks;
mod tokio;
mod transport;
mod properties;

pub use tasks::App as RsbtApp;

pub use crate::tokio::TokioApp as RsbtTokioApp;
pub use error::RsbtError;
pub use result::RsbtResult;
pub use properties::RsbtAppProperties;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
