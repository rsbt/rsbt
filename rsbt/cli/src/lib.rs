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

        // Start web wizard in background if not explicitly disabled
        let config_dir_for_web = cli.config_dir.clone();
        if !cli.no_web {
            println!("Starting web wizard at http://localhost:7878/ (Ctrl+C to skip)");
            std::thread::spawn(move || {
                if let Err(e) = rsbt_web::run_web_wizard(config_dir_for_web) {
                    eprintln!("Web wizard error: {}", e);
                }
            });
        }

        // Run CLI wizard in foreground
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
    /// Disable web wizard on first run
    #[arg(long)]
    no_web: bool,
    #[command(subcommand)]
    command: Command,
}
