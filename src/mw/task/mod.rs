use crate::utils::{DueDate, Status};

pub struct Task {
    pub id: Option<u64>,
    pub status: Status,
}
