mod database;
mod frontend;
mod mw;
mod utils;

use crate::{
    database::sqlite::Sqlite,
    frontend::cli::Cli,
    mw::{config::AppConfig, Middleware},
};

fn main() {
    env_logger::init();
    AppConfig::init(None);
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
