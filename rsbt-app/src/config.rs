use std::path::PathBuf;

pub const RSBT_DIR: &str = ".rsbt";
pub const TORRENTS_TOML: &str = "torrents.toml";
pub const DOWNLOAD_DIR: &str = "download";
pub const TORRENTS_DIR: &str = "torrents";

#[derive(Clone)]
pub struct Config {
    pub config_dir: PathBuf,
}

impl Config {
    pub fn new(custom_config_dir: Option<PathBuf>) -> Self {
        let config_dir = custom_config_dir.unwrap_or_else(default_config_dir);
        Self { config_dir }
    }

    pub fn download_dir(&self) -> PathBuf {
        self.config_dir.join(DOWNLOAD_DIR)
    }

    pub fn torrents_dir(&self) -> PathBuf {
        self.config_dir.join(TORRENTS_DIR)
    }

    pub fn torrents_config(&self) -> PathBuf {
        self.config_dir.join(TORRENTS_TOML)
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.download_dir())?;
        std::fs::create_dir_all(self.torrents_dir())?;
        Ok(())
    }

    pub fn needs_initial_setup(&self) -> bool {
        !self.config_dir.is_dir()
    }
}

pub fn default_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(RSBT_DIR)
}

pub fn need_initial_configuration(custom_config_dir: Option<PathBuf>) -> bool {
    let config_dir = custom_config_dir.unwrap_or_else(default_config_dir);
    !config_dir.is_dir()
}
