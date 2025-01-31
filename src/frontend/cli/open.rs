use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Open {
    pub id: String,
}

impl TryInto<InputCommand> for Open {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.id.parse::<u64>() {
            Ok(id) => Ok(InputCommand::Open(id)),
            Err(_) => Err(FrontEndError::ParseError("status from cli".to_string())),
        }
    }
}
