use std::process::Command;

use super::{BaseProject, Project};

pub struct CliRust {
    pub base: BaseProject,
}

impl Project for CliRust {
    fn build(&self) {
        Command::new("cargo")
            .arg("new")
            .arg(&self.base.name)
            .output()
            .expect("Cannot create cargo project");
        self.base.create_license();
        self.base.create_readme();
    }
}
