use clap::Parser;
use odp_app::Args;
use reqwest::{blocking, Url};

const DOMAIN: &str = "https://data.ct.gov/resource/";

/// Simple program to greet a person

fn main() {
    let args = Args::parse();
    let uri: reqwest::Url = format!("{}{}.csv", DOMAIN, args.fourbyfour)
        .parse::<Url>()
        .expect("Not parseable :C");
    println!("{uri}");
}
