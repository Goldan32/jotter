use crate::{cli::utils::CliError, command::Add as AddCommand};
use std::convert::TryInto;

#[derive(Debug)]
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
