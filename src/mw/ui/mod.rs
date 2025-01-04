use crate::mw::task::Task;

pub trait FrontEndInput {
    fn add(name: String, description: String, duedate: String) -> Task;
}

