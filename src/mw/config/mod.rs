use config::{Config, Environment, File};
use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[allow(unused)]
    #[serde(deserialize_with = "with_expand_envs")]
    pub editor: PathBuf,
    #[allow(unused)]
    #[serde(deserialize_with = "with_expand_envs")]
    pub root_dir: PathBuf,
    #[serde(deserialize_with = "with_expand_envs")]
    pub task_db: PathBuf,
    #[allow(unused)]
    #[serde(deserialize_with = "with_expand_envs")]
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

        let conf = s.build().unwrap().try_deserialize().unwrap();
        Self::create_dirs(&conf);
        let _ = Self::global().set(Arc::new(conf));
    }

    pub fn get() -> Arc<AppConfig> {
        Self::global().get().expect("Config not init").clone()
    }

    fn create_dirs(conf: &AppConfig) {
        fs::create_dir_all(&conf.root_dir).expect("Failed to create root_dir");
        fs::create_dir_all(&conf.work_dir).expect("Failed to create work_dir");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs;
    use std::env;

    #[test]
    fn test_config() {
        unsafe { env::set_var("BJL_WORK_DIR", "target/env_override") }
        AppConfig::init(Some(HashMap::from([(
            "task_db".to_string(),
            "target/init_override.db3".to_string(),
        )])));
        unsafe { env::remove_var("BJL_WORK_DIR") }

        let mut mock_root_dir = dirs::home_dir().unwrap();
        mock_root_dir.push(".local/share/bjl");

        let cfg = AppConfig::get();
        assert_eq!(cfg.root_dir, mock_root_dir);
        assert_eq!(cfg.task_db, PathBuf::from("target/init_override.db3"));
        assert_eq!(cfg.work_dir, PathBuf::from("target/env_override"));
    }
}
