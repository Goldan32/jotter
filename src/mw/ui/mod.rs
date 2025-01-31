use crate::{
    mw::{task::Task, Error},
    utils::Status,
};

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum InputCommand {
    Add(Task),
    Ls(Status),
    Show(u64),
    Open(u64),
}

/// Args in these functions are input received from user (eg. via cli)
/// Outputs are something MW can work with
#[allow(unused)]
pub trait FrontEndCapabilities {
    fn add(name: String, description: String, duedate: String) -> Task;
    fn ls(status: String) -> Status;
    fn show(id: String) -> u64;
    fn open(id: String) -> u64;
}

pub trait FrontEndInput {
    fn new() -> Self;
    fn execute(&self) -> Result<InputCommand, FrontEndError>;
}

#[derive(Debug, PartialEq)]
pub enum FrontEndError {
    NotImplemented(String),
    ParseError(String),
    FsError(String),
    UnknownError,
}

impl Error for FrontEndError {}

impl Display for FrontEndError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(s) => write!(f, "Error parsing {}", s),
            Self::NotImplemented(s) => write!(f, "{} is not implemented", s),
            Self::FsError(s) => write!(f, "Error during fs operation: {}", s),
            Self::UnknownError => write!(f, "Unknown error occured"),
        }
    }
}

pub enum TaskDisplay {
    Full,
    Oneline,
}

pub trait FrontEndOutput {
    fn display_task(&self, t: Task, disp: TaskDisplay);
    fn display_error<T: Error>(&self, e: T) -> i32;
    fn task_editor(&self, t: Task) -> i32;
}
