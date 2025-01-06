mod add;
mod ls;
mod utils;

use crate::mw::ui::InputCommand;
use add::Add;
use clap::{Arg, Command as ClapC};
use ls::Ls;
use std::ffi::OsString;
use std::iter::IntoIterator;

#[allow(dead_code)]
pub fn get_command<I, T>(args: I) -> InputCommand
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
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
        .get_matches_from(args);

    match matches.subcommand() {
        Some(("add", sub_m)) => TryInto::<InputCommand>::try_into(Add {
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
        Some(("ls", sub_m)) => TryInto::<InputCommand>::try_into(Ls {
            status: sub_m
                .get_one::<String>("status")
                .unwrap_or(&String::from("All"))
                .clone(),
        })
        .unwrap(),
        _ => panic!("Pls no"),
    }
}
