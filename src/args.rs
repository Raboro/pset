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
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::MobileAppExpo => write!(f, "mobile app with react native and expo"),
        }
    }
}
