mod utils;

use crate::{
    mw::{
        db::{DatabaseError, DatabaseOps},
        task::Task,
    },
    utils::Status,
};
use chrono::NaiveDate;
use rusqlite::Connection;

pub struct Sqlite {
    conn: Connection,
}

impl Sqlite {
    pub fn open(path: &str) -> Result<Self, DatabaseError> {
        let tmp = Self {
            conn: match Connection::open(path) {
                Ok(c) => c,
                Err(e) => panic!("Error creating rusqlite connection: {:?}", e),
            },
        };
        match tmp.conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                            id INTEGER PRIMARY KEY AUTOINCREMENT,
                            title TEXT NOT NULL,
                            description TEXT,
                            status TEXT,
                            due DATE
                          );",
            (),
        ) {
            Ok(_) => Ok(tmp),
            Err(_) => Err(DatabaseError::CreateTableError),
        }
    }
}

impl DatabaseOps for Sqlite {
    fn open(path: &str) -> Self {
        Sqlite::open(path).expect("Error creating Sqlite object")
    }

    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError> {
        #[allow(unused)]
        if let Some(id) = t.id {
            // Modify
        } else {
            // Create
            let tmp_due: NaiveDate = t
                .due
                .clone()
                .try_into()
                .expect("Error converting DueDate to NaiveDate");
            self.conn
                .execute(
                    "INSERT INTO tasks (title, description, status, due)
                 VALUES (?1, ?2, ?3, ?4)",
                    (&t.title, &t.description, &t.status, &tmp_due),
                )
                .expect("Error inserting new task to database");
        }
        Ok(t)
    }

    #[allow(unused)]
    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError> {
        Ok(Vec::new())
    }
}
