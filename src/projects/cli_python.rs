use super::{BaseProject, Project};

pub struct CliPython {
    pub base: BaseProject,
}

impl Project for CliPython {
    fn build(&self) {}
}
