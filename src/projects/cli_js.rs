use super::{BaseProject, Project};

pub struct CliJs {
    pub base: BaseProject,
}

impl Project for CliJs {
    fn build(&self) {}
}
