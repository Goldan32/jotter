use crate::{frontend::cli::utils::CliError, mw::ui::InputCommand, utils::Status};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Ls {
    pub status: String,
}

impl TryInto<InputCommand> for Ls {
    type Error = CliError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        Ok(InputCommand::Ls(self.status.parse().unwrap()))
    }
}
