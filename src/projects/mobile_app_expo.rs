use super::{BaseProject, Project};

pub struct MobileAppExpo {
    pub base: BaseProject,
}

impl Project for MobileAppExpo {
    fn build(&self) {
        self.base.build();
    }
}
