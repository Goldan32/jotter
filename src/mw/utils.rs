use crate::mw::Error;

use super::db::DatabaseError;

#[derive(Debug, PartialEq)]
pub enum MWError {
    DB(DatabaseError),
}

impl Error for MWError {}

impl std::fmt::Display for MWError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DB(dbe) => write!(f, "Database Error in MW: {}", dbe),
        }
    }
}
