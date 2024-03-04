use super::{BaseProject, Project};

pub struct CliC {
    pub base: BaseProject,
}

impl Project for CliC {
    fn build(&self) {}
}
