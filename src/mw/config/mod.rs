use config::{Config, Environment, File};
use serde::Deserialize;
use serde_with_expand_env::with_expand_envs;
use std::{
    collections::HashMap,
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

        let conf = s.build().unwrap();
        let _ = Self::global().set(Arc::new(conf.try_deserialize().unwrap()));
    }

    pub fn get() -> Arc<AppConfig> {
        Self::global().get().expect("Config not init").clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs;
    use std::env;

    #[test]
    fn test_config() {
        unsafe { env::set_var("BJL_WORK_DIR", "env_override") }
        AppConfig::init(Some(HashMap::from([(
            "task_db".to_string(),
            "init_override".to_string(),
        )])));
        unsafe { env::remove_var("BJL_WORK_DIR") }

        let mut mock_root_dir = dirs::home_dir().unwrap();
        mock_root_dir.push(".local/share/bjl");

        let cfg = AppConfig::get();
        assert_eq!(cfg.root_dir, mock_root_dir);
        assert_eq!(cfg.task_db, PathBuf::from("init_override"));
        assert_eq!(cfg.work_dir, PathBuf::from("env_override"));
    }
}
