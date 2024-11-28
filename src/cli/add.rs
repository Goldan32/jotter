use crate::{cli::utils::CliError, command::Add as AddCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Add {
    pub name: String,
    pub description: String,
    pub date: String,
}

impl TryInto<AddCommand> for Add {
    type Error = CliError;
    fn try_into(self) -> Result<AddCommand, Self::Error> {
        Ok(AddCommand {
            name: self.name,
            description: self.description,
            date: self.date.parse().unwrap(),
        })
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

        let uut: AddCommand = data.try_into().unwrap();

        assert_eq!(
            uut,
            AddCommand {
                name: "UUT Name".to_string(),
                description: "UUT Description".to_string(),
                date: DueDate::Tomorrow,
            }
        )
    }
}
