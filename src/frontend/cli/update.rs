use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Update {
    pub id: String,
    pub field: String,
    pub new_value: String,
}

impl TryInto<InputCommand> for Update {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        match self.id.parse::<u64>() {
            Ok(id) => Ok(InputCommand::Update(id)),
            Err(_) => Err(FrontEndError::ParseError("id from cli".to_string())),
        }
    }
}
