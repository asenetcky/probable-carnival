use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Resource URL
    #[clap(short, long)]
    pub endpoint: reqwest::Url,
}
