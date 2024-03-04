use super::{BaseProject, Project};

pub struct CliTs<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for CliTs<'a> {
    fn build(&self) {}
}
