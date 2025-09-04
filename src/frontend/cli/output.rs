use crate::{
    frontend::cli::Cli,
    mw::{
        config::AppConfig,
        task::Task,
        ui::{FrontEndError, FrontEndOutput},
    },
};
use regex::Regex;
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};
use {
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::crossterm::style::Color,
    termimad::*,
};

impl FrontEndOutput for Cli {
    fn display_task(&self, t: Task) {
        self.display_task_long_md(t);
    }

    fn display_task_list(&self, v: Vec<Task>) {
        self.display_task_list_md(v);
    }

    fn display_error<T: crate::mw::Error>(&self, e: T) -> i32 {
        log::error!("{}", e);
        1
    }

    fn task_editor(&self, mut t: Task) -> Result<Task, FrontEndError> {
        const DESC_FILE_NAME: &str = "description.md";
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
                File::create(format!("{}/{}", &editor_root_str, DESC_FILE_NAME)).unwrap();

            // Write description to file
            if let Err(e) = description_file.write_all(&t.description.unwrap().into_bytes()) {
                return Err(FrontEndError::FsError(e.to_string()));
            }
        }

        // Open editor
        let status = Command::new("nvim")
            .arg(DESC_FILE_NAME)
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
                File::open(format!("{}/{}", &editor_root_str, DESC_FILE_NAME)).unwrap();
            if let Err(e) = description_file.read_to_string(&mut readback) {
                return Err(FrontEndError::FsError(e.to_string()));
            }
        }

        // Return edited task
        t.description = Some(readback);
        Ok(t)
    }
}

static TASK_TEMPLATE_FULL: &str = r#"

# ${task-id} - ${task-title}

|:-:|:-:|
|**Status**|${task-status}|
|-|-|
|**Due**|${task-due}|
|-|-|

${task-description}

"#;

static TASK_TEMPLATE_LIST: &str = r#"
|:-:|:-:|:-:|:-:|
|**ID**|**Title**|**Status**|**Due**|
${each-task
|:-:|:-|:-:|:-:|
| ${task-id} | ${task-title} | ${task-status} | ${task-due} |
}
|-|-|-|-|
"#;

impl Cli {
    fn display_task_long_md(&self, t: Task) {
        // Regex to replace `- ` at the start of lines with `* ` (list markers)
        let list_re = Regex::new(r"(?m)^(\s*)- ").expect("Can't make regex");
        let chkbx_re = Regex::new(r"\[(x| )\]").expect("Can't make regex");
        let mut mod_desc: String;
        if let Some(d) = t.description {
            mod_desc = d;
            mod_desc = list_re.replace_all(&mod_desc, "$1* ").to_string();
            mod_desc = chkbx_re
                .replace_all(&mod_desc, |caps: &regex::Captures| match &caps[1] {
                    "x" => "✓",
                    " " => "☐",
                    _ => panic!("Impossible"),
                })
                .to_string();
        } else {
            mod_desc = "".to_string();
        }
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("task-id", format!("{}", t.id.unwrap_or(0u64)))
            .set("task-title", t.title)
            .set("task-status", t.status)
            .set("task-due", t.due)
            .set_lines_md("task-description", mod_desc);
        let skin = make_skin();
        let template = TextTemplate::from(TASK_TEMPLATE_FULL);
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
        print!("{}", fmt_text);
    }

    fn display_task_list_md(&self, v: Vec<Task>) {
        let mut expander = OwningTemplateExpander::new();
        for t in v {
            expander
                .sub("each-task")
                .set("task-id", format!("{}", t.id.unwrap_or(0u64)))
                .set("task-title", t.title)
                .set("task-status", t.status)
                .set("task-due", t.due);
        }
        let skin = make_skin();
        let template = TextTemplate::from(TASK_TEMPLATE_LIST);
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
        print!("{}", fmt_text);
    }
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(Color::AnsiValue(178));
    skin.headers[0].align = termimad::Alignment::Left;
    skin.headers[1].set_fg(Color::Magenta);
    skin.bold.set_fg(Color::Yellow);
    skin.italic.set_fg(Color::Cyan);
    skin.inline_code.set_fg(Color::Red);
    skin.bullet = StyledChar::from_fg_char(Color::Magenta, '▸');
    skin.list_items_indentation_mode = ListItemsIndentationMode::Block;
    skin.scrollbar.thumb.set_fg(Color::AnsiValue(178));
    skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
    skin
}
