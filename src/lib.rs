use clap::Parser;

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
