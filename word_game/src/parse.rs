use std::{num::ParseIntError, str::FromStr};

pub fn convert<T>(s: &str) -> Result<T, ParseIntError>
    where
        T: FromStr<Err = ParseIntError>,
    {
        s.trim().parse::<T>()
    }
