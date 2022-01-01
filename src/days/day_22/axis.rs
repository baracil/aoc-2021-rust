use std::str::FromStr;
use itertools::Itertools;
use crate::days::day_22::command::Command;
use crate::days::day_22::coordinate::Coordinate;
use crate::days::day_22::cube::Cube;
use crate::days::day_22::range::Range;

use crate::{AOCResult, parse_input};

pub struct Axis {
    coordinates: Vec<Coordinate>
}


impl Axis {

    pub fn length(&self, index:usize) -> usize {
        self.coordinates[index].size
    }

    pub fn new(commands:&[Command<i32>], range_getter:impl Fn(&Cube<i32>) -> &Range<i32>) -> Self {
        let coordinates:Vec<Coordinate> = commands.iter()
            .map(|Command{cube,..}| range_getter(cube))
            .flat_map(|r| [r.min,r.max].into_iter())
            .sorted()
            .unique()
            .tuple_windows::<(_,_)>()
            .map(|(vmin,vmax)| Coordinate::new_with_min_max(vmin, vmax))
            .collect();

        Axis{coordinates}
    }

    pub fn transform_range(&self, range:&Range<i32>) -> AOCResult<Range<usize>> {
        let min_idx = self.coordinates.iter().position(|c| c.value == range.min).unwrap_or_else(|| self.coordinates.len());
        let max_idx = self.coordinates.iter().position(|c| c.value == range.max).unwrap_or_else(|| self.coordinates.len());

        Ok(Range::of(min_idx, max_idx))
    }

}
