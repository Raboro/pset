use crate::args::ProjectType;

mod mobile_app_expo;

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
            ProjectType::CliC => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::CliRust => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::CliPython => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::CliJS => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::CliTS => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::ObsidianPlugin => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::BasicJava => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::BasicPython => Box::new(mobile_app_expo::MobileAppExpo { base }),
            ProjectType::FullStackSpringBootAngular => {
                Box::new(mobile_app_expo::MobileAppExpo { base })
            }
        }
    }
}
