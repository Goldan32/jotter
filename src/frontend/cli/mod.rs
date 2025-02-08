mod add;
mod ls;
mod open;
mod show;

use crate::mw::{
    config::AppConfig,
    task::Task,
    ui::{FrontEndError, FrontEndInput, FrontEndOutput, InputCommand, TaskDisplay},
};
use add::Add;
use clap::{Arg, Command as ClapC};
use ls::Ls;
use open::Open;
use show::Show;
use std::{
    ffi::OsString,
    fs::File,
    io::{Read, Write},
    iter::IntoIterator,
    process::Command,
};

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
                    "{} - {} | {} | {}",
                    t.id.unwrap_or(0),
                    t.title,
                    t.status,
                    t.due
                );
            }
        }
    }

    fn display_error<T: crate::mw::Error>(&self, e: T) -> i32 {
        eprintln!("{}", e);
        1
    }

    fn task_editor(&self, mut t: Task) -> Result<Task, FrontEndError> {
        // Create location
        let config = AppConfig::get();
        let mut editor_root = config.work_dir.clone();
        editor_root.push(t.id.unwrap().to_string());
        if let Err(e) = std::fs::create_dir_all(&editor_root) {
            return Err(FrontEndError::FsError(e.to_string()));
        }

        let editor_root_str = editor_root.to_str().unwrap();
        {
            // Create file
            let mut description_file =
                File::create(format!("{}/description", &editor_root_str)).unwrap();

            // Write description to file
            if let Err(e) = description_file.write_all(&t.description.unwrap().into_bytes()) {
                return Err(FrontEndError::FsError(e.to_string()));
            }
        }

        // Open editor
        let status = Command::new("nvim")
            .arg("description")
            .current_dir(&editor_root)
            .status()
            .unwrap();
        if !status.success() {
            return Err(FrontEndError::FsError(format!(
                "Error code: {:?}",
                status.code()
            )));
        }

        // Read back file contents
        let mut readback: String = String::new();
        {
            let mut description_file =
                File::open(format!("{}/description", &editor_root_str)).unwrap();
            if let Err(e) = description_file.read_to_string(&mut readback) {
                return Err(FrontEndError::FsError(e.to_string()));
            }
        }

        // Return edited task
        t.description = Some(readback);
        Ok(t)
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
                .short_flag('s')
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
                .short_flag('o')
                .about("Open task with selected id for editing")
                .arg(
                    Arg::new("id")
                        .help("Id of task to open")
                        .required(true)
                        .index(1),
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
        Some((s, _)) => Err(FrontEndError::NotImplemented(s.to_string())),
        _ => Err(FrontEndError::UnknownError),
    }
}
