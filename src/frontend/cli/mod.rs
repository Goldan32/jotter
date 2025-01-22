mod add;
mod ls;
mod show;

use crate::mw::{
    task::Task,
    ui::{FrontEndError, FrontEndInput, FrontEndOutput, InputCommand, TaskDisplay},
};
use add::Add;
use clap::{Arg, Command as ClapC};
use ls::Ls;
use show::Show;
use std::ffi::OsString;
use std::iter::IntoIterator;

pub struct Cli;

impl Cli {
    pub fn new() -> Self {
        Cli
    }
}

impl FrontEndInput for Cli {
    fn new() -> Self {
        Cli::new()
    }
    fn execute(&self) -> Result<InputCommand, FrontEndError> {
        get_command(std::env::args_os())
    }
}

impl FrontEndOutput for Cli {
    fn display_task(&self, t: Task, disp: TaskDisplay) {
        match disp {
            TaskDisplay::Full => {
                println!("{}", t);
            }
            TaskDisplay::Oneline => {
                println!(
                    "{} - {} | {:?} | {:?}",
                    t.id.unwrap_or(0),
                    t.title,
                    t.status,
                    t.due
                );
            }
        }
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
                .short_flag('a')
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
        .subcommand(
            ClapC::new("show")
                .short_flag('s')
                .about("Show task with selected id")
                .arg(
                    Arg::new("id")
                        .help("Id of task to display")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches_from(args);

    match matches.subcommand() {
        Some(("add", sub_m)) => Ok(TryInto::<InputCommand>::try_into(Add {
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
        .unwrap()),
        Some(("ls", sub_m)) => Ok(TryInto::<InputCommand>::try_into(Ls {
            status: sub_m
                .get_one::<String>("status")
                .unwrap_or(&String::from("All"))
                .clone(),
        })
        .unwrap()),
        Some(("show", sub_m)) => Ok(TryInto::<InputCommand>::try_into(Show {
            id: sub_m
                .get_one::<String>("id")
                .expect("Missing task id")
                .clone(),
        })
        .unwrap()),
        _ => Err(FrontEndError::NotImplemented),
    }
}
