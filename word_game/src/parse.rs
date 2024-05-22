use std::str::FromStr;

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn parse(s: &str) -> u8 {
        let from_u8 = s.trim()
            .parse::<u8>()
            .map_err(|_| format!("Error while trying to parse to u8."))
            .expect("Expected a valid whole number between 0-255");

        let input = from_u8.to_string();

        match u8::from_str(&input) {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}
