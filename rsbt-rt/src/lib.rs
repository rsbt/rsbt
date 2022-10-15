/*!
# rsbt-rt description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-rt = "0.1"
```

*/

pub trait Runtime {}

mod tokio_runtime;

#[cfg(feature = "tokio_1")]
pub type DefaultRuntime = tokio_runtime::TokioRuntime;

#[cfg(not(feature = "tokio_1"))]
compile_error!("You must enable tokio_1 feature, as it is only one supported in the moment");
