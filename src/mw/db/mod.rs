use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    UnknownError,
}

pub trait DatabaseOps {
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError>;
    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError>;
}
