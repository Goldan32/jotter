use datetime::LocalDate;

pub trait Add {
    fn add(name: &str, time: LocalDate, description: &str) -> u32;
}

// pub trait Modify {}
// pub trait Delete {}
// pub trait Progress {}
// pub trait Show {}
// pub trait Open {}
