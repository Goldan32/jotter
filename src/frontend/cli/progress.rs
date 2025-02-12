use crate::mw::ui::{FrontEndError, InputCommand};
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct Progress {
    pub id: String,
    pub new_status: Option<String>,
}

impl TryInto<InputCommand> for Progress {
    type Error = FrontEndError;
    fn try_into(self) -> Result<InputCommand, Self::Error> {
        let id = match self.id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(FrontEndError::ParseError("id from cli".to_string())),
        };
        let status = if let Some(s) = self.new_status {
            match s.parse() {
                Ok(ss) => Some(ss),
                Err(_) => return Err(FrontEndError::ParseError("status from cli".to_string())),
            }
        } else {
            None
        };
        Ok(InputCommand::Progress(id, status))
    }
}
