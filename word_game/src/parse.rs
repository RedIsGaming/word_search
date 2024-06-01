use std::num::ParseIntError;

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn new(s: &str) -> Result<u8, ParseIntError> {
        match s.trim().parse::<u8>().map_err(|e| e) {
            Ok(num) => Ok(num),
            Err(err) => return Err(err),
        }
    }
}
