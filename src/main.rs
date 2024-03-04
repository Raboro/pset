use clap::Parser;

use crate::projects::{BaseProject, ProjectFactory};

mod args;
mod fs;
mod projects;
mod templates;

fn main() {
    let args = args::Args::parse();
    let project = ProjectFactory::create(
        BaseProject::new(args.name, 2024, "Marius Wörfel"),
        args.project_type,
    );

    project.build();
}
