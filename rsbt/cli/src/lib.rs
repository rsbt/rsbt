use clap::Parser;

use command::Command;
use rsbt_app::AppStatus;

mod command;

/// rsbt client.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> AppStatus {
    dotenv::dotenv().ok();
    env_logger::init();

    use crate::command::Runnable;

    match Cli::try_parse().map(|cli| cli.command).map(Runnable::run) {
        Ok(_) => AppStatus::Success,
        Err(err) => {
            let _ = err.print();

            if err.use_stderr() {
                AppStatus::CommandLineFailStderr
            } else {
                AppStatus::CommandLineFail
            }
        }
    }
}
