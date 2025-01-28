mod database;
mod frontend;
mod mw;
mod utils;

use crate::{database::sqlite::Sqlite, frontend::cli::Cli, mw::Middleware};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let executor: Middleware<Cli, Sqlite> = match Middleware::new() {
        Ok(ex) => ex,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };
    let ret = executor.main();
    std::process::exit(ret)
}
