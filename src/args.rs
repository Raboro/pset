use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the project
    #[arg(short, long)]
    pub project_name: String,
}
