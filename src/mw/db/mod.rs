use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    UnknownError,
}

pub trait DatabaseOps {
    fn insert_or_modify(t: Task) -> Result<Task, DatabaseError>;
    fn list(status: Status) -> Result<Vec<Task>, DatabaseError>;
}
