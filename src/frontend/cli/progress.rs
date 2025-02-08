use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Progress {
    pub id: String,
}

impl TryInto<InputCommand> for Progress {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.id.parse::<u64>() {
            Ok(id) => Ok(InputCommand::Progress(id)),
            Err(_) => Err(FrontEndError::ParseError("id from cli".to_string())),
        }
    }
}
