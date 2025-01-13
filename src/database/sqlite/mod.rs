//TODO: Remove this allow
#![allow(unused)]

mod insert_or_modify;
mod utils;

use crate::{
    mw::{
        db::{DatabaseError, DatabaseOps},
        task::Task,
    },
    utils::Status,
};

use chrono::NaiveDate;
use rusqlite::{params, Connection, Result as rResult};

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn open(path: &str) -> Result<Self, DatabaseError> {
        let mut tmp = Self {
            conn: Connection::open(path).unwrap(),
        };
        match tmp.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            title TEXT NOT NULL,
                            description TEXT,
                            status INTEGER,
                            due DATE
                          );",
            (),
        ) {
            Ok(_) => Ok(tmp),
            Err(_) => Err(DatabaseError::CreateConnectionError),
        }
    }
}

impl DatabaseOps for Sqlite {
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError> {
        if let Some(id) = t.id {
            // Modify
        } else {
            // Create
            let tmp_due: NaiveDate = t.due.clone().try_into().unwrap();
            self.conn
                .execute(
                    "INSERT INTO tasks (title, description, status, due)
                 VALUES (?1, ?2, ?3, ?4)",
                    (&t.title, &t.description, &t.status, &tmp_due),
                )
                .unwrap();
        }
        Ok(t)
    }

    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError> {
        Ok(Vec::new())
    }
}
