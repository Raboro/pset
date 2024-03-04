use super::{BaseProject, Project};

pub struct CliRust {
    pub base: BaseProject,
}

impl Project for CliRust {
    fn build(&self) {}
}
