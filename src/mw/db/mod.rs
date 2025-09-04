use crate::{
    mw::{task::Task, Error},
    utils::Status,
};

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    OpenError(String, String),
    CreateTableError(String),
    QueryError(String),
    QueryMapError(String),
    ConvertError(String, String),
    InsertError(String),
    EditError(String, String),
    PragmaError,
    #[allow(unused)]
    UnknownError,
}

impl Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenError(s, e) => write!(f, "Error opening {} ({})", s, e),
            Self::CreateTableError(s) => write!(f, "Error creating table: {}", s),
            Self::QueryError(e) => write!(f, "Error with query: {}", e),
            Self::QueryMapError(e) => write!(f, "Error mapping query result: {}", e),
            Self::ConvertError(e, s) => write!(f, "Error converting {} to {}", e, s),
            Self::InsertError(e) => write!(f, "Error inserting into database: {}", e),
            Self::EditError(field, e) => write!(f, "Error editing '{}' field: {}", field, e),
            Self::PragmaError => write!(f, "Error executing a pragma"),
            Self::UnknownError => write!(f, "Unknown database error occured"),
        }
    }
}

pub trait DatabaseOps: Sized {
    fn open(path: &str) -> Result<Self, DatabaseError>;
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError>;
    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError>;
    fn get_by_id(&self, id: u64) -> Result<Task, DatabaseError>;
    fn create_epic(
        &self,
        title: String,
        tag: Option<String>,
        description: Option<String>,
    ) -> Result<(), DatabaseError>;
}
