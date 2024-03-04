use super::{BaseProject, Project};

pub struct CliPython<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for CliPython<'a> {
    fn build(&self) {}
}
