use super::{add::Add, ls::Ls, open::Open, progress::Progress, show::Show, update::Update};
use crate::{
    frontend::cli::Cli,
    mw::ui::{FrontEndError, FrontEndInput, InputCommand},
};
use clap::{Arg, Command as ClapC};
use std::{ffi::OsString, iter::IntoIterator};

impl FrontEndInput for Cli {
    fn new() -> Self {
        Cli::new()
    }

    fn execute(&self) -> Result<InputCommand, FrontEndError> {
        get_command(std::env::args_os())
    }
}

pub fn get_command<I, T>(args: I) -> Result<InputCommand, FrontEndError>
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
                .visible_alias("a")
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
                        .required(true),
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
        .subcommand(
            ClapC::new("show")
                .visible_alias("s")
                .about("Show task with selected id")
                .arg(
                    Arg::new("id")
                        .help("Id of task to display")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            ClapC::new("open")
                .visible_alias("o")
                .about("Open task with selected id for editing")
                .arg(
                    Arg::new("id")
                        .help("Id of task to open")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            ClapC::new("progress")
                .visible_alias("p")
                .about("Progress the task status to the next one")
                .arg(
                    Arg::new("id")
                        .help("Id of task to open")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("status")
                        .short('s')
                        .help("New status to apply")
                        .required(false),
                ),
        )
        .subcommand(
            ClapC::new("update")
                .visible_alias("u")
                .about("Update the title of a task")
                .arg(
                    Arg::new("id")
                        .help("Id of task to update")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("new_title")
                        .help("New title for task")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches_from(args);

    match matches.subcommand() {
        Some(("add", sub_m)) => TryInto::<InputCommand>::try_into(Add {
            name: sub_m.get_one::<String>("name").unwrap().clone(),
            description: sub_m
                .get_one::<String>("description")
                .unwrap_or(&String::from(""))
                .to_string(),
            date: sub_m.get_one::<String>("time").unwrap().clone(),
        }),
        Some(("ls", sub_m)) => TryInto::<InputCommand>::try_into(Ls {
            status: sub_m
                .get_one::<String>("status")
                .unwrap_or(&String::from("todo"))
                .clone(),
        }),
        Some(("show", sub_m)) => TryInto::<InputCommand>::try_into(Show {
            id: sub_m
                .get_one::<String>("id")
                .expect("Missing task id")
                .clone(),
        }),
        Some(("open", sub_m)) => TryInto::<InputCommand>::try_into(Open {
            id: sub_m
                .get_one::<String>("id")
                .expect("Missing task id")
                .clone(),
        }),
        Some(("progress", sub_m)) => TryInto::<InputCommand>::try_into(Progress {
            id: sub_m
                .get_one::<String>("id")
                .expect("Missing task id")
                .clone(),
            new_status: sub_m.get_one::<String>("status").cloned(),
        }),
        Some(("update", sub_m)) => TryInto::<InputCommand>::try_into(Update {
            id: sub_m
                .get_one::<String>("id")
                .expect("Missing task id")
                .clone(),
            new_title: sub_m
                .get_one::<String>("new_title")
                .expect("Missing new title")
                .clone(),
        }),
        Some((s, _)) => Err(FrontEndError::NotImplemented(s.to_string())),
        _ => Err(FrontEndError::UnknownError),
    }
}
