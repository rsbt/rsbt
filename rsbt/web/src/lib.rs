pub mod wizard;

pub fn run_web_wizard(config_dir: Option<std::path::PathBuf>) -> Result<(), rsbt_app::AppError> {
    use rsbt_app::{Config, need_initial_configuration};

    let config = Config::new(config_dir);
    wizard::serve_wizard(config)
}