use clap::Parser;
use odp_app::Args;

const DOMAIN: &str = "https://data.ct.gov/resource/";

/// Simple program to greet a person

fn main() {
    let args = Args::parse();

    let uri = format!("{}{}", DOMAIN, args.fourbyfour);

    println!("Hello {:#?}!", uri);
}
