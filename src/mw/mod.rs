pub mod db;
pub mod task;
pub mod ui;

use crate::mw::{
    db::DatabaseOps,
    ui::{FrontEndInput, InputCommand},
};

pub struct Middleware<T: FrontEndInput, U: DatabaseOps> {
    ui: T,
    db: U,
}

impl<T: FrontEndInput, U: DatabaseOps> Middleware<T, U> {
    pub fn new() -> Self {
        Self {
            ui: T::new(),
            db: U::open(&(std::env::var("BJL_DATABASE").expect("BJL_DATABASE must be set"))),
        }
    }

    pub fn main(&self) {
        let command: InputCommand = self.ui.execute();
        match command {
            InputCommand::Add(t) => {
                let inserted_task = self
                    .db
                    .insert_or_modify(t)
                    .expect("Failed insert_or_modify operation");
                println!("{:?}", inserted_task);
            }
            InputCommand::Ls(s) => {
                let v = self.db.list(s).expect("Failed list operation");
                println!("{:?}", v);
            }
            #[allow(unreachable_patterns)]
            _ => println!("Not implemented yet"),
        }
    }
}
