use std::ops::{Add, Sub};
use std::str::FromStr;
use crate::days::day_22::range::Range;

use crate::parse_input;

#[derive(Clone)]
pub struct Cube<T> {
    pub x_range: Range<T>,
    pub y_range: Range<T>,
    pub z_range: Range<T>,
}


impl FromStr for Cube<i32> {
    type Err = String;

    fn from_str(cube_as_str: &str) -> Result<Self, Self::Err> {
        let ranges  = cube_as_str.split(',')
            .map(|r| r.parse::<Range<i32>>())
            .collect::<Result<Vec<Range<i32>>,String>>()?;

        Ok(Cube { x_range: ranges[0], y_range: ranges[1], z_range: ranges[2] })
    }
}