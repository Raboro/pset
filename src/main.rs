use clap::Parser;

mod args;
mod templates;

fn main() {
    let args = args::Args::parse();
    println!("{}", args);
}
