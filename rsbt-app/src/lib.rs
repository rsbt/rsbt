mod error;
mod status;

pub use crate::error::AppError;
pub use crate::status::AppStatus;

#[derive(Default)]
pub struct App {}

#[derive(Default)]
pub struct BlockingApp {
    app: App,
}
