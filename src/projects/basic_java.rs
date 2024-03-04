use super::{BaseProject, Project};

pub struct BasicJava<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for BasicJava<'a> {
    fn build(&self) {}
}
