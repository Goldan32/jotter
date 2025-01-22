use crate::utils::{DueDate, Status};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<u64>,
    pub status: Status,
    pub due: DueDate,
    pub title: String,
    pub description: Option<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{} - {}\nStatus: {:?}\nDue: {:?}\n\n{}",
            self.id.unwrap_or(0),
            self.title,
            self.status,
            self.due,
            // TODO: Get rid of this clone
            self.description.clone().unwrap_or("N/A".to_string())
        )
    }
}
