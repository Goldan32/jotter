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

use rusqlite::{params, Connection, Result as rResult};

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn open(path: &str) -> Result<Self, DatabaseError> {
        Ok(Self {
            conn: Connection::open(path).unwrap(),
        })
    }
}

impl DatabaseOps for Sqlite {
    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError> {
        if let Some(id) = t.id {
            // Modify
        } else {
            // Create
            self.conn
                .execute(
                    "INSERT INTO tasks (title, description, status, due)
                 VALUES (?1, ?2, ?3, ?4)",
                    (&t.title, &t.description, &t.status, &t.due),
                )
                .unwrap();
        }
        Ok(t)
    }

    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError> {
        Ok(Vec::new())
    }
}
