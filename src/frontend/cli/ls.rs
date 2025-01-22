use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Ls {
    pub status: String,
}

impl TryInto<InputCommand> for Ls {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.status.parse() {
            Ok(s) => Ok(InputCommand::Ls(s)),
            Err(_) => Err(FrontEndError::ParseError("status from cli".to_string())),
        }
    }
}
