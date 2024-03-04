use super::{BaseProject, Project};

pub struct CliJs<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for CliJs<'a> {
    fn build(&self) {}
}
