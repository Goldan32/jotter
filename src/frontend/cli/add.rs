use crate::{
    mw::{
        task::Task,
        ui::{FrontEndError, InputCommand},
    },
    utils::{DueDate, Status},
};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Add {
    pub name: String,
    pub description: String,
    pub date: String,
}

impl TryInto<InputCommand> for Add {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.date.parse::<DueDate>() {
            Ok(date) => Ok(InputCommand::Add(Task {
                title: self.name,
                description: Some(self.description),
                due: date,
                id: None,
                status: Status::Todo,
            })),
            Err(_) => Err(FrontEndError::ParseError("date from cli".to_string())),
        }
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

        let uut: InputCommand = data
            .try_into()
            .expect("Error making InputCommand from Cli::Add");

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
