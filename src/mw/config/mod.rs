use config::{Config, Environment, File};
use serde_derive::Deserialize;
use std::{
    collections::HashMap,
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

    /// Initialize the config, configs won't change after this call
    ///
    /// cfg: Configs in this map will overwrite any other config sources
    pub fn init(cfg: Option<HashMap<String, String>>) {
        let mut s = Config::builder()
            .add_source(File::with_name("config-default.toml"))
            .add_source(Environment::with_prefix("bjl"));

        if let Some(c) = cfg {
            for (key, value) in c {
                s = s.set_override(key, value).unwrap();
            }
        }

        let conf = s.build().unwrap();

        println!("{:#?}", conf);
        let _ = Self::global().set(Arc::new(conf.try_deserialize().unwrap()));
    }

    pub fn get() -> Arc<AppConfig> {
        Self::global().get().expect("Config not init").clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        AppConfig::init(Some(HashMap::from([(
            "task_db".to_string(),
            "init_override".to_string(),
        )])));
        let cfg = AppConfig::get();
        assert!(cfg.root_dir.exists());
        assert_eq!(cfg.task_db, PathBuf::from("init_override"));
    }
}
