mod add;
mod ls;
mod utils;

use crate::command::{Add as AddCommand, Command as AppCommand, Ls as LsCommand};
use add::Add;
use clap::{Arg, Command as ClapC};
use ls::Ls;

#[allow(dead_code)]
pub fn get_command() -> AppCommand {
    let matches = ClapC::new("budget-jira")
        .about("A CLI task handler written in Rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            ClapC::new("add")
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
            ClapC::new("ls").about("List tasks").arg(
                Arg::new("status")
                    .help("List task with this status")
                    .required(false)
                    .index(1),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => AppCommand::Add(
            TryInto::<AddCommand>::try_into(Add {
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
            })
            .unwrap(),
        ),
        Some(("ls", sub_m)) => AppCommand::Ls(
            TryInto::<LsCommand>::try_into(Ls {
                status: sub_m
                    .get_one::<String>("status")
                    .unwrap_or(&String::from("All"))
                    .clone(),
            })
            .unwrap(),
        ),
        _ => panic!("Pls no"),
    }
}
