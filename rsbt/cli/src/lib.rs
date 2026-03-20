use clap::Parser;

use command::Command;
use rsbt_app::{need_initial_configuration, AppError, AppStatus, Config};

mod command;
mod wizard;

pub fn run() -> AppStatus {
    dotenvy::dotenv().ok();
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    let Ok(_) = tracing::subscriber::set_global_default(subscriber) else {
        return AppStatus::CommandLineFail;
    };

    let cli = Cli::parse();

    // Check if first run and run wizard if needed
    if need_initial_configuration(cli.config_dir.clone()) {
        let config = Config::new(cli.config_dir.clone());
        match wizard::run_wizard(config) {
            Ok(_) => {
                println!("Configuration complete!");
            }
            Err(e) => {
                eprintln!("Wizard failed: {}", e);
                return AppStatus::CommandLineFail;
            }
        }
    }

    use crate::command::Runnable;

    match cli.command.run() {
        Ok(_) => AppStatus::Success,
        Err(AppError::Config(msg)) => {
            eprintln!("{}", msg);
            AppStatus::CommandLineFail
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            AppStatus::CommandLineFail
        }
    }
}

/// rsbt client.
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    /// Custom configuration directory
    #[arg(short, long)]
    config_dir: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command: Command,
}
