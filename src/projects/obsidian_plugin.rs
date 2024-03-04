use super::{BaseProject, Project};

pub struct ObsidianPlugin {
    pub base: BaseProject,
}

impl Project for ObsidianPlugin {
    fn build(&self) {}
}
