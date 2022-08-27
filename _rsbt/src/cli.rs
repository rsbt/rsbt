use std::path::PathBuf;
use structopt::StructOpt;

/// RSBT
#[derive(StructOpt)]
pub struct Cli {
    /// Custom configuration directory.
    ///
    /// Default: $HOME/.rsbt
    #[structopt(long)]
    pub config_dir: Option<PathBuf>,
    /// Web server listen address.
    #[structopt(long, default_value = "127.0.0.1:8080")]
    pub listen_addr: String,
    /// Wizard listen address.
    ///
    /// Default: use --listen-addr value.
    #[structopt(long)]
    pub wizard_listen_addr: Option<String>,

    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
