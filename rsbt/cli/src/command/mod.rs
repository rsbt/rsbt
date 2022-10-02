use clap::Parser;

use rsbt_app::AppError;

mod download_command;

use download_command::DownloadCommand;

#[derive(Debug, Parser)]
pub enum Command {
    Download(DownloadCommand),
}

pub trait Runnable {
    fn run(self) -> Result<(), AppError>;
}

impl Runnable for Command {
    fn run(self) -> Result<(), AppError> {
        match self {
            Command::Download(download_command) => download_command.run(),
        }
    }
}
