use crate::args::ProjectType;

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

pub struct BaseProject<'a> {
    name: &'a str,
    year: u16,
    author: &'a str,
}

pub trait Project {
    fn build(&self);
}
pub struct ProjectFactory {}

impl ProjectFactory {
    fn create(base: BaseProject<'static>, project_type: ProjectType) -> Box<dyn Project> {
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
