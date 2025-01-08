use crate::utils::Status;

use rusqlite::types::Value;
use rusqlite::{types::FromSql, types::FromSqlError, types::ToSql, types::ValueRef, Result};

// Implement ToSql
impl ToSql for Status {
    fn to_sql(&self) -> Result<Value> {
        Ok(Value::Integer(match self {
            Status::Done => 0,
            Status::Todo => 1,
            Status::Archived => 2,
        }))
    }
}

// Implement FromSql
impl FromSql for Status {
    fn column_result(value: ValueRef) -> Result<Self, FromSqlError> {
        match value.as_i64()? {
            0 => Ok(Status::Done),
            1 => Ok(Status::Todo),
            2 => Ok(Status::Archived),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
