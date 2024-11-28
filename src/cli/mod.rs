mod add;
mod ls;
mod utils;

use add::Add;
use clap::{Arg, ArgAction, Command, Subcommand};
use ls::Ls;
use std::fmt::Debug;

// TEST: Example function taking Add struct
fn test_subcommand<T>(args: T)
where
    T: Debug,
{
    println!("{:?}", args);
}

#[allow(dead_code)]
pub fn cli_app() {
    let matches = Command::new("budget-jira")
        .about("A CLI task handler written in Rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add a new task")
                .arg(
                    Arg::new("name")
                        .help("Title of the task")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .help("Task description")
                        .short('d')
                        .long("description")
                        .required(false),
                )
                .arg(
                    Arg::new("time")
                        .help("Due date")
                        .short('t')
                        .long("time")
                        .required(false)
                        .default_value("indefinite"),
                ),
        )
        .subcommand(
            Command::new("ls").about("List tasks").arg(
                Arg::new("status")
                    .help("List task with this status")
                    .required(false)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => test_subcommand(Add {
            name: sub_m
                .get_one::<String>("name")
                .expect("Missing task name")
                .clone(),
            description: sub_m
                .get_one::<String>("description")
                .unwrap_or(&String::from("No description"))
                .to_string(),
            date: sub_m
                .get_one::<String>("time")
                .unwrap_or(&String::from("Indefinite"))
                .clone(),
        }),
        Some(("ls", sub_m)) => test_subcommand(Ls {
            status: sub_m
                .get_one::<String>("status")
                .unwrap_or(&String::from("All"))
                .clone(),
        }),
        _ => panic!("Pls no"),
    }
}
