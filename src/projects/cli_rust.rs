use super::{BaseProject, Project};

pub struct CliRust<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for CliRust<'a> {
    fn build(&self) {}
}
