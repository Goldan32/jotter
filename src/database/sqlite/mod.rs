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
        let conn = match Connection::open(path) {
            Ok(c) => c,
            Err(e) => return Err(DatabaseError::OpenError(path.to_string(), e.to_string())),
        };
        let tmp = Self { conn };
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

    fn get_task_by_id(&self, id: u64) -> Result<Task, DatabaseError> {
        let mut stmt = match self.conn.prepare(
            "SELECT id, title, description, status, due
                FROM tasks
                WHERE id = :task_id
                ORDER BY id
                LIMIT 1;",
        ) {
            Ok(s) => s,
            Err(e) => return Err(DatabaseError::QueryError(e.to_string())),
        };
        let rows = match stmt.query_map(named_params! {":task_id": id}, |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
                due: {
                    let d: NaiveDate = row.get(4).unwrap();
                    d.try_into().expect("Someone messed with the database")
                },
            })
        }) {
            Ok(r) => r,
            Err(e) => return Err(DatabaseError::QueryMapError(e.to_string())),
        };
        let mut v: Vec<Task> = rows.map(|x| x.unwrap()).collect();
        match v.len() {
            0 => Err(DatabaseError::QueryError(
                "No such id found in database".to_string(),
            )),
            _ => Ok(v.remove(0)),
        }
    }

    fn set_field<T: rusqlite::ToSql>(
        &self,
        id: u64,
        field: &str,
        new_value: T,
    ) -> Result<(), DatabaseError> {
        match self.conn.execute(
            &format!(
                "UPDATE tasks
                 SET {} = ?1
                 WHERE id = ?2",
                field
            ),
            (&id, &new_value),
        ) {
            Ok(_) => Ok(()),
            Err(e) => return Err(DatabaseError::EditError(field.to_string(), e.to_string())),
        }
    }
}

impl DatabaseOps for Sqlite {
    fn open(path: &str) -> Result<Self, DatabaseError> {
        Sqlite::open(path)
    }

    fn insert_or_modify(&self, t: Task) -> Result<Task, DatabaseError> {
        if let Some(id) = t.id {
            // Modify
            let stored_task = self.get_task_by_id(id)?;
            if stored_task.title != t.title {
                self.set_field(t.id.expect("Impossible"), "title", &t.title)?;
            }
            if stored_task.description != t.description {
                self.set_field(t.id.expect("Impossible"), "description", &t.description)?;
            }
        } else {
            // Create
            let tmp_due: NaiveDate = match t.due.clone().try_into() {
                Ok(d) => d,
                Err(_) => {
                    return Err(DatabaseError::ConvertError(
                        t.due.to_string(),
                        "Date format".to_string(),
                    ))
                }
            };
            match self.conn.execute(
                "INSERT INTO tasks (title, description, status, due)
                 VALUES (?1, ?2, ?3, ?4)",
                (&t.title, &t.description, &t.status, &tmp_due),
            ) {
                Ok(task) => task,
                Err(e) => return Err(DatabaseError::InsertError(e.to_string())),
            };
        }
        Ok(t)
    }

    fn list(&self, status: Status) -> Result<Vec<Task>, DatabaseError> {
        let mut stmt = match self.conn.prepare(
            "SELECT id, title, status, due
                    FROM tasks 
                    WHERE status = :status
                    ORDER BY id;",
        ) {
            Ok(s) => s,
            Err(e) => return Err(DatabaseError::QueryError(e.to_string())),
        };
        let rows = match stmt.query_map(named_params! {":status": status.to_string()}, |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                description: None,
                status: row.get(2)?,
                due: {
                    let d: NaiveDate = row.get(3).unwrap();
                    d.try_into().expect("Someone messed with the database")
                },
            })
        }) {
            Ok(t) => t,
            Err(e) => return Err(DatabaseError::QueryMapError(e.to_string())),
        };
        let v: Vec<Task> = rows.map(|x| x.unwrap()).collect();
        Ok(v)
    }

    fn get_by_id(&self, id: u64) -> Result<Task, DatabaseError> {
        self.get_task_by_id(id)
    }
}
