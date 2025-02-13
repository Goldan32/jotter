use crate::{
    frontend::cli::Cli,
    mw::{
        config::AppConfig,
        task::Task,
        ui::{FrontEndError, FrontEndOutput, TaskDisplay},
    },
};
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};

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
