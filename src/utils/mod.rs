//use datetime::LocalDate;
use std::str;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug)]
pub struct ConvertError;

// TODO: Change Other to Localdate
#[derive(Debug)]
pub enum DueDate {
    Today,
    Tomorrow,
    EndOfWeek,
    Other(String),
}

#[derive(EnumString, Debug)]
pub enum Status {
    #[strum(serialize = "done", ascii_case_insensitive)]
    Done,
    #[strum(serialize = "todo", ascii_case_insensitive)]
    Todo,
    #[strum(serialize = "archived", ascii_case_insensitive)]
    Archived,
}

impl FromStr for DueDate {
    type Err = ConvertError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "today" => Ok(Self::Today),
            "tomorrow" => Ok(Self::Tomorrow),
            "endofweek" | "eow" => Ok(Self::EndOfWeek),
            s => Ok(Self::Other(s.to_string())),
        }
    }
}
