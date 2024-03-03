use core::fmt;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the project
    #[arg(short, long)]
    pub name: String,

    /// Type of the project
    #[arg(short, long, value_enum)]
    pub project_type: ProjectType,

    /// Generate GitHub action for CI (ci.yml), default false
    #[arg(short, long)]
    pub ci: bool,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Generate '{}' as {} {}",
            self.name,
            self.project_type,
            if self.ci {
                "and with ci github action"
            } else {
                ""
            }
        )
    }
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum ProjectType {
    MobileAppExpo,
    CliC,
    CliRust,
    CliPython,
    CliJS,
    CliTS,
    ObsidianPlugin,
    BasicJava,
    BasicPython,
    FullStackSpringBootAngular,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::MobileAppExpo => write!(f, "Mobile app with React Native and expo"),
            ProjectType::CliC => write!(f, "Cli app with C"),
            ProjectType::CliRust => write!(f, "Cli app with Rust"),
            ProjectType::CliPython => write!(f, "Cli app with Python"),
            ProjectType::CliJS => write!(f, "Cli app with JavaScript"),
            ProjectType::CliTS => write!(f, "Cli app with TypeScript"),
            ProjectType::ObsidianPlugin => write!(f, "Obsidian Plugin with TypeScript"),
            ProjectType::BasicJava => write!(f, "Basic Java project with Maven"),
            ProjectType::BasicPython => write!(f, "Basic Python project"),
            ProjectType::FullStackSpringBootAngular => write!(
                f,
                "Full Stack Web app with Java Spring Boot Maven and Angular TypeScript in docker compose"
            ),
        }
    }
}
