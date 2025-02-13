pub mod config;
pub mod db;
pub mod task;
pub mod ui;
pub mod utils;

use crate::mw::{
    config::AppConfig,
    db::DatabaseOps,
    ui::{FrontEndInput, FrontEndOutput, InputCommand},
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
                self.ui.display_task_list(tasks);
                0
            }
            InputCommand::Show(id) => {
                let task = match self.db.get_by_id(id) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                self.ui.display_task(task);
                0
            }
            InputCommand::Open(id) => {
                let task = match self.db.get_by_id(id) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                let edited_task = match self.ui.task_editor(task) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                if let Err(e) = self.db.insert_or_modify(edited_task) {
                    return self.ui.display_error(e);
                }
                0
            }
            InputCommand::Progress(id, status) => {
                let mut task = match self.db.get_by_id(id) {
                    Ok(t) => t,
                    Err(e) => return self.ui.display_error(e),
                };
                if let Err(e) = task.progress_status(status) {
                    return self.ui.display_error(e);
                }
                if let Err(e) = self.db.insert_or_modify(task) {
                    return self.ui.display_error(e);
                }
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
