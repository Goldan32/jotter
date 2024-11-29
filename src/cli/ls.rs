use crate::{cli::utils::CliError, command::Get as GetCommand};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Ls {
    pub status: String,
}

impl TryInto<GetCommand> for Ls {
    type Error = CliError;
    fn try_into(self) -> Result<GetCommand, Self::Error> {
        Ok(GetCommand {
            status: self.status.parse().unwrap(),
        })
    }
}
