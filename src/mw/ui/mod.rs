use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputCommand {
    Add(Task),
    Ls(Status),
}

#[allow(unused)]
pub trait FrontEndCapabilities {
    fn add(name: String, description: String, duedate: String) -> Task;
    fn ls(status: String) -> Status;
}

pub trait FrontEndInput {
    fn execute(&self) -> InputCommand;
}
