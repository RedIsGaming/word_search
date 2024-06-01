use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn new<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<T, ParseIntError> {
        match s.trim().parse::<T>().map_err(|e| e) {
            Ok(num) => Ok(num),
            Err(err) => return Err(err),
        }
    }
}
