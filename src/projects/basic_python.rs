use super::{BaseProject, Project};

pub struct BasicPython<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for BasicPython<'a> {
    fn build(&self) {}
}
