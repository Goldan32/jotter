pub mod config;
pub mod db;
pub mod task;
pub mod ui;
pub mod utils;

use crate::mw::{
    config::AppConfig,
    db::DatabaseOps,
    ui::{FrontEndInput, FrontEndOutput, InputCommand, TaskDisplay},
    utils::MWError,
};

pub struct Middleware<T: FrontEndInput, U: DatabaseOps> {
    ui: T,
    db: U,
}

pub trait Error: std::fmt::Display {}

impl<T: FrontEndInput + FrontEndOutput, U: DatabaseOps> Middleware<T, U> {
    pub fn new() -> Result<Self, MWError> {
        let ui = T::new();
        let config = AppConfig::get();
        let db_path = config.task_db.to_str().unwrap();
        let db = match U::open(db_path) {
            Ok(db) => db,
            Err(e) => return Err(MWError::DB(e)),
        };
        Ok(Self { ui, db })
    }

    pub fn main(&self) -> i32 {
        let command: InputCommand = match self.ui.execute() {
            Ok(c) => c,
            Err(e) => return self.ui.display_error(e),
        };
        match command {
            InputCommand::Add(t) => match self.db.insert_or_modify(t) {
                Ok(_) => 0,
                Err(e) => self.ui.display_error(e),
            },
            InputCommand::Ls(s) => {
                let tasks = match self.db.list(s) {
                    Ok(v) => v,
                    Err(e) => return self.ui.display_error(e),
                };
                for t in tasks {
                    self.ui.display_task(t, TaskDisplay::Oneline);
                }
                0
            }
            InputCommand::Show(id) => {
                let task = match self.db.get_by_id(id) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                self.ui.display_task(task, TaskDisplay::Full);
                0
            }
            InputCommand::Open(id) => {
                let task = match self.db.get_by_id(id) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                self.ui.task_editor(task);
                0
            }
            #[allow(unreachable_patterns)]
            _ => {
                eprintln!("Not implemented yet");
                2
            }
        }
    }
}
