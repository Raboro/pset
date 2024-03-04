use super::{BaseProject, Project};

pub struct FullStackSpringBootAngular<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for FullStackSpringBootAngular<'a> {
    fn build(&self) {}
}
