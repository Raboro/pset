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

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum ProjectType {
    MobileAppExpo,
}
