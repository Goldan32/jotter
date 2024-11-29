use crate::utils::{DueDate, Status};

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<u32>,
    pub status: Option<Status>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub date: Option<DueDate>,
}

impl Task {
    pub fn new() -> Self {
        Task {
            id: None,
            status: None,
            name: None,
            description: None,
            date: None,
        }
    }
}
