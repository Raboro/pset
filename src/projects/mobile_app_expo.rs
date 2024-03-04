use super::{BaseProject, Project};

pub struct MobileAppExpo<'a> {
    pub base: BaseProject<'a>,
}

impl<'a> Project for MobileAppExpo<'a> {
    fn build(&self) {}
}
