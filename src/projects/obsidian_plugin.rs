use std::{path::PathBuf, process::Command};

use crate::fs;

use super::{BaseProject, Project};

pub struct ObsidianPlugin {
    pub base: BaseProject,
}

impl Project for ObsidianPlugin {
    fn build(&self) {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/Raboro/Obsidian-Plugin-Template.git")
            .output()
            .expect("Cannot clone obsidian template");

        fs::create_file(PathBuf::from("./Obsidian-Plugin-Template/info.md"), String::from("Before doing anything rename template and removed .git folder, because it's a clone")).expect("Cannot create info.md");
    }
}
