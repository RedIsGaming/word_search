use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn convert<T: FromStr<Err = ParseIntError>>(s: &str) -> Result<T, ParseIntError> {
        match s.trim().parse::<T>() {
            Ok(num) => Ok(num),
            Err(err) => Err(err),
        }
    }
}
