use crate::utils::{DueDate, Status};

pub struct Add {
    pub name: String,
    pub date: DueDate,
    pub description: String,
}

pub struct Ls {
    pub status: Status,
}
