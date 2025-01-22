use crate::{frontend::cli::utils::CliError, mw::ui::InputCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Show {
    pub id: String,
}

impl TryInto<InputCommand> for Show {
    type Error = CliError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        Ok(InputCommand::Show(self.id.parse::<u64>().unwrap()))
    }
}
