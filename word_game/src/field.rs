use std::str::FromStr;

#[derive(Debug)]
pub struct Field;

impl Field {
    pub fn parse(s: &str) -> u8 {
        let parse = s.trim()
            .parse::<u8>()
            .map_err(|_| format!("Error while trying to parse to u8."))
            .expect("Expected a valid whole number between 0-255");

        let input = parse.to_string();

        match u8::from_str(&input) {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}
