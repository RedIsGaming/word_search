use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Parse;

impl Parse {
    pub fn convert<T>(s: &str) -> Result<T, ParseIntError>
    where
        T: FromStr<Err = ParseIntError>,
    {
        s.trim().parse::<T>()
    }
}
