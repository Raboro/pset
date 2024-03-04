use super::{BaseProject, Project};

pub struct CliTs {
    pub base: BaseProject,
}

impl Project for CliTs {
    fn build(&self) {}
}
