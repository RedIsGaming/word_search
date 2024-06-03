use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn convert<T>(s: &str) -> Result<T, ParseIntError> 
        where T: FromStr<Err = ParseIntError>, 
    {
        match s.trim().parse::<T>() {
            Ok(num) => Ok(num),
            Err(err) => Err(err),
        }
    }
}
