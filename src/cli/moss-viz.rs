use clap::Parser;

mod args;
mod engines;
mod formats;
use args::CliArgs;

fn main() {
    let args = CliArgs::parse_from(wild::args());

    println!("{:#?}", args);
}
