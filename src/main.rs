use clap::Parser;

mod args;
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
    println!("{}", args);
}
