pub mod db;
pub mod task;
pub mod ui;

use crate::{
    frontend::cli::Cli,
    mw::ui::{FrontEndInput, InputCommand},
};

pub struct Middleware<T: FrontEndInput> {
    ui: T,
}

impl Middleware<Cli> {
    pub fn new() -> Self {
        Self { ui: Cli::new() }
    }

    pub fn main(&self) {
        let command: InputCommand = self.ui.execute();
        println!("{:?}", command);
    }
}
