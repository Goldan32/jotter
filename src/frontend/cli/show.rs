use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Show {
    pub id: String,
}

impl TryInto<InputCommand> for Show {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.id.parse::<u64>() {
            Ok(id) => Ok(InputCommand::Show(id)),
            Err(_) => Err(FrontEndError::ParseError("id from cli".to_string())),
        }
    }
}
