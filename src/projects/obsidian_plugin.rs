use super::{BaseProject, Project};

pub struct ObsidianPlugin<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for ObsidianPlugin<'a> {
    fn build(&self) {}
}
