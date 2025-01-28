pub mod db;
pub mod task;
pub mod ui;

use crate::mw::{
    db::DatabaseOps,
    ui::{FrontEndInput, FrontEndOutput, InputCommand, TaskDisplay},
};

pub struct Middleware<T: FrontEndInput, U: DatabaseOps> {
    ui: T,
    db: U,
}

pub trait Error: std::fmt::Display {}

impl<T: FrontEndInput + FrontEndOutput, U: DatabaseOps> Middleware<T, U> {
    pub fn new() -> Self {
        Self {
            ui: T::new(),
            db: U::open(&(std::env::var("BJL_DATABASE").expect("BJL_DATABASE must be set")))
                .unwrap(),
        }
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
            #[allow(unreachable_patterns)]
            _ => {
                eprintln!("Not implemented yet");
                2
            }
        }
    }
}
