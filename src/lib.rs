use clap::Parser;
use reqwest::{Error, Response};

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    pub fourbyfour: String,
    // /// Number of times to greet
    // #[clap(short, long, default_value = "1")]
    // pub count: u8,
}

pub async fn grab_data(uri: reqwest::Url) -> Result<(), reqwest::Error> {
    let body = reqwest::get(uri).await?.text().await?;

    println!("body = {body:?}");
    Ok(())
}
