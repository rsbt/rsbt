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
}