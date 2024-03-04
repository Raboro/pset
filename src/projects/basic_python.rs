use super::{BaseProject, Project};

pub struct BasicPython {
    pub base: BaseProject,
}

impl Project for BasicPython {
    fn build(&self) {}
}
