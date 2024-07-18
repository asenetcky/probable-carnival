// use clap::Parser;
// use reqwest::{Error, Response};

// #[derive(Parser, Debug)]
// #[clap(version, about)]
// pub struct Args {
//     /// Name of the person to greet
//     #[clap(short, long)]
//     pub fourbyfour: String,
//     // /// Number of times to greet
//     // #[clap(short, long, default_value = "1")]
//     // pub count: u8,
// }

// pub async fn grab_data(uri: reqwest::Url) -> Result<(), reqwest::Error> {
//     let body = reqwest::get(uri).await?.text().await?;

//     println!("body = {body:?}");
//     Ok(())
// }

/* A Rewrite of RSocrata package by Hugh J. Devlin, Ph. D. 2013-08-28 in Rust
RSocrata is an interface to data hosted online in Socrata (now Tyler Technologies Open Data Platform) data repositories
*/
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    /// Resource URL
    #[clap(short, long)]
    pub endpoint: reqwest::Url,
}

pub fn field_name(human_name: String) -> String {
    let re = Regex::new("\\.").unwrap();
    re.replace_all(&human_name, "_").to_string().to_lowercase()
}

pub fn is_four_by_four(fourbyfour: String) -> bool {
    let string_length = fourbyfour.chars().count();

    if string_length != 9 {
        return false;
    }

    let re = Regex::new("[[:alnum:]]{4}-[[:alnum:]]{4}").unwrap();
    if !re.is_match(&fourbyfour) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_true_is_true() {
        let good_string = "test-test".to_string();
        assert_eq!(is_four_by_four(good_string), true);
    }

    #[test]
    fn four_false_is_false() {
        let bad_string = "testtest".to_string();
        assert_ne!(is_four_by_four(bad_string), true);
    }

    // #[test]
    // #[should_panic]
    // fn not_string() {
    //     let not_string = 12345;
    //     assert_ne!(is_four_by_four(not_string), true);
    // }

    #[test]
    fn field_readable() {
        let string = "No.Periods.Here".to_string();
        assert_eq!(field_name(string), "no_periods_here".to_string());
    }
}
