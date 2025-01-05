use crate::utils::{DueDate, Status};

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<u64>,
    pub status: Status,
    pub due: DueDate,
    pub title: String,
    pub description: Option<String>,
}
