use std::process::{ExitCode, Termination};

use crate::AppError;

pub enum AppStatus {
    Success,
    CommandLineFailStderr,
    CommandLineFail,
    Failure(AppError),
}

impl Termination for AppStatus {
    fn report(self) -> ExitCode {
        match self {
            Self::Success | Self::CommandLineFail => ExitCode::SUCCESS,
            Self::CommandLineFailStderr => ExitCode::from(2),
            Self::Failure(err) => {
                log::error!("{err}");
                ExitCode::FAILURE
            }
        }
    }
}
