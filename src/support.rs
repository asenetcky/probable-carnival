use regex::Regex;
use reqwest::{IntoUrl, Request};

pub fn field_name(human_name: String) -> String {
    let re = Regex::new("\\.").unwrap();
    re.replace_all(&human_name, "_").to_string().to_lowercase()
}

fn no_deniro(s: &str) -> f64 {
    let s = s.trim_start_matches('$');
    s.parse::<f64>().expect("Not parseable to a float")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_deniro_test() {
        let s = "$123.45";
        assert_eq!(no_deniro(s), 123.45);
    }
}
