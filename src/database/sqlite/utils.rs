use crate::utils::Status;

use rusqlite::types::Value;
use rusqlite::{
    types::FromSql, types::FromSqlError, types::ToSql, types::ToSqlOutput, types::ValueRef, Error,
    Result,
};

// Implement ToSql
impl ToSql for Status {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, Error> {
        let s = format!("{:?}", self).to_lowercase();
        Ok(ToSqlOutput::Owned(Value::Text(s)))
    }
}

// Implement FromSql
impl FromSql for Status {
    fn column_result(value: ValueRef) -> Result<Self, FromSqlError> {
        let s = value.as_str()?;
        Ok(Status::try_from(s).expect("Error converting database entry to Status"))
    }
}
