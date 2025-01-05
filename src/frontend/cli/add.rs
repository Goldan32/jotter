use crate::{
    frontend::cli::utils::CliError,
    mw::{task::Task, ui::InputCommand},
    utils::Status,
};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Add {
    pub name: String,
    pub description: String,
    pub date: String,
}

impl TryInto<InputCommand> for Add {
    type Error = CliError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        Ok(InputCommand::Add(Task {
            title: self.name,
            description: Some(self.description),
            due: self.date.parse().unwrap(),
            id: None,
            status: Status::Todo,
        }))
    }
}

mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::DueDate;

    #[test]
    fn test_try_into_add_command() {
        let data = Add {
            name: "UUT Name".to_string(),
            description: "UUT Description".to_string(),
            date: "Tomorrow".to_string(),
        };

        let uut: InputCommand = data.try_into().unwrap();

        assert_eq!(
            uut,
            InputCommand::Add(Task {
                title: "UUT Name".to_string(),
                description: Some("UUT Description".to_string()),
                due: DueDate::Tomorrow,
                id: None,
                status: Status::Todo,
            })
        )
    }
}
