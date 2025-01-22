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

    pub fn main(&self) {
        let command: InputCommand = self.ui.execute().unwrap();
        match command {
            InputCommand::Add(t) => {
                let _inserted_task = self
                    .db
                    .insert_or_modify(t)
                    .expect("Failed insert_or_modify operation");
            }
            InputCommand::Ls(s) => {
                let v = self.db.list(s).expect("Failed list operation");
                for t in v {
                    self.ui.display_task(t, TaskDisplay::Oneline);
                }
            }
            InputCommand::Show(id) => {
                let t = self.db.get_by_id(id).unwrap();
                self.ui.display_task(t, TaskDisplay::Full);
            }
            #[allow(unreachable_patterns)]
            _ => eprintln!("Not implemented yet"),
        }
    }
}
