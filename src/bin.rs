mod database;
mod frontend;
mod mw;
mod utils;

use mw::db::DatabaseOps;

fn main() {
    // let executor = crate::mw::Middleware::new();
    // executor.main();
    let db = crate::database::sqlite::Sqlite::open("/home/goldan/projects/tmp/sample.db").unwrap();
    let _ = db.insert_or_modify(crate::mw::task::Task {
        id: None,
        status: utils::Status::Todo,
        title: "First task".to_string(),
        due: utils::DueDate::Tomorrow,
        description: Some("First description".to_string()),
    });
}
