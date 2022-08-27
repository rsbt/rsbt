use std::path::PathBuf;

pub(crate) fn need_initial_configuration(custom_config_dir: Option<PathBuf>) -> bool {
    let config_dir = if let Some(config_dir) = custom_config_dir {
        config_dir
    } else {
        let home_dir = dirs::home_dir().unwrap_or_else(|| ".".into());
        home_dir.join(".rsbt")
    };

    !config_dir.is_dir()
}
