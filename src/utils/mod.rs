use chrono::{
    naive::{Days, NaiveDate},
    offset::Local,
    Datelike,
};
use std::{convert::TryInto, str::FromStr};
use strum_macros::{Display, EnumString};

#[derive(Debug)]
pub struct ConvertError;

#[derive(Debug, PartialEq, Clone, Display)]
pub enum DueDate {
    Today,
    Tomorrow,
    EndOfWeek,
    Other(String),
}

#[derive(EnumString, Display, Debug, PartialEq, Clone)]
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
            Self::Other(s) => {
                Ok(NaiveDate::parse_from_str(&s, "%Y-%-m-%-d").expect("Bad date format given"))
            }
        }
    }
}

impl TryFrom<NaiveDate> for DueDate {
    type Error = ConvertError;
    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        let today = Local::now().date_naive();
        let tomorrow = today
            .checked_add_days(Days::new(1))
            .expect("Error adding one day to current date");
        let day = today.weekday().num_days_from_monday();
        let end_of_week = today
            .checked_add_days(Days::new(day as u64 + 4u64))
            .expect("Error adding 4 days to current date");
        match value {
            d if d == end_of_week => Ok(Self::EndOfWeek),
            d if d == today => Ok(Self::Today),
            d if d == tomorrow => Ok(Self::Tomorrow),
            other => Ok(Self::Other(other.format("%Y-%m-%d").to_string())),
        }
    }
}
