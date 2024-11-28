use crate::utils::{DueDate, Status};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Add(Add),
    Ls(Ls),
}

#[derive(Debug, PartialEq)]
pub struct Add {
    pub name: String,
    pub date: DueDate,
    pub description: String,
}

#[derive(Debug)]
pub struct Ls {
    pub status: Status,
}
