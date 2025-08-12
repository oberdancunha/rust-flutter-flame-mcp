use env_logger::Env;
use log::{LevelFilter, error, info};

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .filter_level(LevelFilter::Info)
            .init();

        Self {}
    }

    pub fn log(message: &str) {
        info!("{}", message);
    }

    pub fn log_error(message: &str) {
        error!("!!!{}", message);
    }
}
