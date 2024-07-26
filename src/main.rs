use clap::Parser;
use odp_app::*;

pub mod cli;

fn main() {
    let args = Args::parse();
    let printthis = args.endpoint;
    println!("{printthis}");
}
