use crate::mw::task::Task;
use crate::utils::Status;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputCommand {
    Add(Task),
    Ls(Status),
}

pub trait FrontEndCapabilities {
    fn add(name: String, description: String, duedate: String) -> Task;
    fn ls(status: String) -> Status;
}

pub trait FrontEndInput {
    fn execute() -> InputCommand;
}
