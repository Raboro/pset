use super::{BaseProject, Project};

pub struct CliC<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for CliC<'a> {
    fn build(&self) {}
}
