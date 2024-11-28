use crate::{cli::utils::CliError, command::Ls as LsCommand};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Ls {
    pub status: String,
}

impl TryInto<LsCommand> for Ls {
    type Error = CliError;
    fn try_into(self) -> Result<LsCommand, Self::Error> {
        Ok(LsCommand {
            status: self.status.parse().unwrap(),
        })
    }
}
