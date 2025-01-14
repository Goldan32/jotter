mod database;
mod frontend;
mod mw;
mod utils;

use crate::{database::sqlite::Sqlite, frontend::cli::Cli, mw::Middleware};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let executor: Middleware<Cli, Sqlite> = Middleware::new();
    executor.main();
}
