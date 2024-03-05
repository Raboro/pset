use crate::{args::ProjectType, fs, templates};

mod basic_java;
mod basic_python;
mod cli_c;
mod cli_js;
mod cli_python;
mod cli_rust;
mod cli_ts;
mod full_stack_spring_boot_angular;
mod mobile_app_expo;
mod obsidian_plugin;

pub struct BaseProject {
    name: String,
    year: u16,
    author: &'static str,
}

impl BaseProject {
    pub fn new(name: String, year: u16, author: &'static str) -> Self {
        Self { name, year, author }
    }

    pub fn build(&self) {
        self.create_dir();
        self.create_readme();
        self.create_license();
    }

    fn create_dir(&self) {
        fs::create_dir(self.name.as_str()).expect("Project Folder could not be created");
    }

    fn create_readme(&self) {
        let readme = templates::Template::new(
            "readme",
            "md",
            None,
            &self.name,
            templates::basics::ReadMe {
                project_name: self.name.as_str(),
            },
        );
        fs::create_file(readme.to_path_buf(), readme.render().unwrap_or_default())
            .expect("Readme cannot be created");
    }

    fn create_license(&self) {
        let license = templates::Template::new(
            "license",
            "md",
            None,
            &self.name,
            templates::basics::License {
                year: self.year,
                author: self.author,
            },
        );
        fs::create_file(license.to_path_buf(), license.render().unwrap_or_default())
            .expect("License cannot be created");
    }
}

pub trait Project {
    fn build(&self);
}
pub struct ProjectFactory {}

impl ProjectFactory {
    pub fn create(base: BaseProject, project_type: ProjectType) -> Box<dyn Project> {
        match project_type {
            ProjectType::MobileAppExpo => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::CliC => Box::new(cli_c::CliC { base }),
            ProjectType::CliRust => Box::new(cli_rust::CliRust { base }),
            ProjectType::CliPython => Box::new(cli_python::CliPython { base }),
            ProjectType::CliJS => Box::new(cli_js::CliJs { base }),
            ProjectType::CliTS => Box::new(cli_ts::CliTs { base }),
            ProjectType::ObsidianPlugin => Box::new(obsidian_plugin::ObsidianPlugin { base }),
            ProjectType::BasicJava => Box::new(basic_java::BasicJava { base }),
            ProjectType::BasicPython => Box::new(basic_python::BasicPython { base }),
            ProjectType::FullStackSpringBootAngular => {
                Box::new(full_stack_spring_boot_angular::FullStackSpringBootAngular { base })
            }
        }
    }
}
