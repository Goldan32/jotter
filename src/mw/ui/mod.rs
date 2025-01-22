use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
pub enum InputCommand {
    Add(Task),
    Ls(Status),
    Show(u64),
}

/// Args in these functions are input received from user (eg. via cli)
/// Outputs are something MW can work with
#[allow(unused)]
pub trait FrontEndCapabilities {
    fn add(name: String, description: String, duedate: String) -> Task;
    fn ls(status: String) -> Status;
    fn show(id: String) -> u64;
}

pub trait FrontEndInput {
    fn new() -> Self;
    fn execute(&self) -> Result<InputCommand, FrontEndError>;
}

#[derive(Debug, PartialEq)]
pub enum FrontEndError {
    NotImplemented,
    ParseError,
}

pub enum TaskDisplay {
    Full,
    Oneline,
}

pub trait FrontEndOutput {
    fn display_task(&self, t: Task, disp: TaskDisplay);
}
