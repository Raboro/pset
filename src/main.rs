use clap::Parser;

mod args;
mod projects;
mod templates;

fn main() {
    let args = args::Args::parse();
    let license = templates::Template::new(
        "license",
        "md",
        None,
        templates::basics::License {
            author: "Marius Wörfel",
            year: 2024,
        },
    );
    print!("{}", license.render().unwrap_or_default());

    let readme = templates::Template::new(
        "README",
        "md",
        None,
        templates::basics::ReadMe {
            project_name: &args.name,
        },
    );

    println!("{}", readme.render().unwrap_or_default());

    println!("{}", args);
}
