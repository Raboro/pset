use clap::Parser;

mod args;

mod template;

fn main() {
    let args = args::Args::parse();
    println!("{}", args);
}
