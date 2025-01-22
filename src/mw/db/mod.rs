use crate::{
    mw::{task::Task, Error},
    utils::Status,
};

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    OpenError(String),
    CreateTableError,
    #[allow(unused)]
    UnknownError,
}

impl Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenError(s) => write!(f, "Error opening {}", s),
            Self::CreateTableError => write!(f, "Error creating table"),
            Self::UnknownError => write!(f, "Unknown database error occured"),
        }
    }
}

pub trait DatabaseOps: Sized {
    fn open(path: &str) -> Result<Self, DatabaseError>;
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError>;
    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError>;
    fn get_by_id(&self, id: u64) -> Result<Task, DatabaseError>;
}
