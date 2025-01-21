mod utils;

use crate::{
    mw::{
        db::{DatabaseError, DatabaseOps},
        task::Task,
    },
    utils::Status,
};
use chrono::NaiveDate;
use rusqlite::{named_params, Connection};

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

    fn get_task_by_id(&self, id: u64) -> Task {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, title, description, status, due
                FROM tasks
                WHERE id = :task_id
                ORDER BY id
                LIMIT 1;",
            )
            .expect("Error querying tasks");
        let rows = stmt
            .query_map(named_params! {":task_id": id}, |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: row.get(3)?,
                    due: {
                        let d: NaiveDate = row.get(4).unwrap();
                        d.try_into().unwrap()
                    },
                })
            })
            .unwrap();
        let mut v: Vec<Task> = rows.map(|x| x.unwrap()).collect();
        v.remove(0)
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

    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, title, status, due
                    FROM tasks ORDER BY id;",
            )
            .unwrap();
        let rows = stmt
            .query_map(named_params! {":status": status.try_into()}, |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: None,
                    status: row.get(2)?,
                    due: {
                        let d: NaiveDate = row.get(4).unwrap();
                        d.try_into().unwrap()
                    },
                })
            })
            .unwrap();
        let v: Vec<Task> = rows.map(|x| x.unwrap()).collect();
        Ok(v)
    }

    fn get_by_id(&self, id: u64) -> Result<Task, DatabaseError> {
        Ok(self.get_task_by_id(id))
    }
}
