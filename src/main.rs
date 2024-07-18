use clap::Parser;
use odp_app::*;

fn main() {
    let args = Args::parse();
    let printthis = args.endpoint;
    println!("{printthis}");
}
