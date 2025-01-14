use chrono::{
    naive::{Days, NaiveDate},
    offset::Local,
    Datelike,
};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use strum_macros::{Display, EnumString};

#[derive(Debug)]
pub struct ConvertError;

#[derive(Debug, PartialEq, Clone)]
pub enum DueDate {
    Today,
    Tomorrow,
    EndOfWeek,
    Other(String),
}

#[derive(EnumString, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Status {
    Done,
    Todo,
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

impl TryInto<NaiveDate> for DueDate {
    type Error = ConvertError;
    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        let today = Local::now().date_naive();
        match self {
            Self::Today => Ok(today),
            Self::Tomorrow => Ok(today
                .checked_add_days(Days::new(1))
                .expect("Error adding one day to current date")),
            Self::EndOfWeek => {
                let day = today.weekday().num_days_from_monday();
                Ok(today
                    .checked_add_days(Days::new(day as u64 + 4u64))
                    .expect("Error adding 4 days to current date"))
            }
            Self::Other(s) => panic!("Error making date from {}, yet!", s),
        }
    }
}
