use crate::utils::{DueDate, Status};

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Command {
    Put(Put),
    Get(Get),
}

#[derive(Debug, PartialEq)]
pub struct Put {
    pub name: String,
    pub date: DueDate,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct Get {
    pub status: Status,
}
