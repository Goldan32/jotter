use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[allow(unused)]
    pub editor: PathBuf,
    #[allow(unused)]
    pub root_dir: PathBuf,
    pub task_db: PathBuf,
    #[allow(unused)]
    pub work_dir: PathBuf,
}

impl AppConfig {
    fn global() -> &'static OnceLock<Arc<AppConfig>> {
        static INSTANCE: OnceLock<Arc<AppConfig>> = OnceLock::new();
        &INSTANCE
    }

    pub fn init() {
        let s = Config::builder()
            .add_source(File::with_name("example-config.toml"))
            .add_source(Environment::with_prefix("bjl"))
            .build()
            .unwrap();

        println!("{:?}", s);
        Self::global()
            .set(Arc::new(s.try_deserialize().unwrap()))
            .expect("Already init");
    }

    pub fn get() -> Arc<AppConfig> {
        Self::global().get().expect("Config not init").clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_init_and_get() {
        AppConfig::init();
        let cfg = AppConfig::get();
        assert_eq!(cfg.task_db, PathBuf::from(r"task_db_path"));
    }
}
