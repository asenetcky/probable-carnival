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
use reqwest::{IntoUrl, Request};

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

fn no_deniro(s: &str) -> f64 {
    let s = s.trim_start_matches('$');
    s.parse::<f64>().expect("Not parseable to a float")
}

pub fn ls_socrata(url_string: String) {
    let parsed_url =
        reqwest::Url::parse(url_string.as_str()).expect("Does not appear to  be a valid URL.");

    let response = reqwest::get(parsed_url);
}

// async fn get_response(
//     url: reqwest::Url,
//     email: Option<String>,
//     password: Option<String>,
// ) -> Result<reqwest::Response, reqwest::Error> {
//     if email == None && password == None {
//         let response = reqwest::get(url).await?.text().await?;
//         println!("{response:?}");
//     }
//     Ok(());
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_deniro_test() {
        let s = "$123.45";
        assert_eq!(no_deniro(s), 123.45);
    }

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
