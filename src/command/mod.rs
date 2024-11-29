use crate::{
    task::Task,
    utils::{ConvertError, DueDate, Status},
};
use std::convert::TryInto;

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
