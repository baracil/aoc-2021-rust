use std::str::FromStr;
use crate::days::day_22::cube::Cube;
use crate::days::day_22::range::Range;
use crate::try_parse_input;

#[derive(Clone)]
pub struct Command<T> {
    pub light_state:bool,
    pub cube:Cube<T>,
}

impl Command<i32> {
    pub(crate) fn is_in_fifty_range(&self) -> bool {
        self.cube.x_range.is_in_fifty_range()
        && self.cube.y_range.is_in_fifty_range()
        && self.cube.z_range.is_in_fifty_range()
    }
}

impl <T> Command<T> {
    pub fn x_range(&self) -> &Range<T> {
        &self.cube.x_range
    }
    pub fn y_range(&self) -> &Range<T> {
        &self.cube.y_range
    }
    pub fn z_range(&self) -> &Range<T> {
        &self.cube.z_range
    }
}

impl FromStr for Command<i32> {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (light_state,cube_as_str) = line.split_once(' ')
            .ok_or_else(|| format!("Cannot parse line {}",line))?;


        let light_state = light_state == "on";
        let cube = cube_as_str.parse::<Cube<i32>>()?;

        Ok(Command { light_state, cube })
    }
}