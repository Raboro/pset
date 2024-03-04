use super::{BaseProject, Project};

pub struct BasicJava {
    pub base: BaseProject,
}

impl Project for BasicJava {
    fn build(&self) {}
}
