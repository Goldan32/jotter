use crate::{cli::utils::CliError, command::Put as PutCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Add {
    pub name: String,
    pub description: String,
    pub date: String,
}

impl TryInto<PutCommand> for Add {
    type Error = CliError;
    fn try_into(self) -> Result<PutCommand, Self::Error> {
        Ok(PutCommand {
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

        let uut: PutCommand = data.try_into().unwrap();

        assert_eq!(
            uut,
            PutCommand {
                name: "UUT Name".to_string(),
                description: "UUT Description".to_string(),
                date: DueDate::Tomorrow,
            }
        )
    }
}
