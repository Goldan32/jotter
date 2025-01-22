use crate::{mw::task::Task, utils::Status};

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    CreateTableError,
    #[allow(unused)]
    UnknownError,
}

pub trait DatabaseOps {
    fn open(path: &str) -> Self;
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError>;
    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError>;
    fn get_by_id(&self, id: u64) -> Result<Task, DatabaseError>;
}
