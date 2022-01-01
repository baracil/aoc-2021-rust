use std::cmp::Ordering;
use std::ops::{Add, RangeInclusive, Sub};
use std::str::FromStr;

use crate::parse_input;

#[derive(Clone,Copy)]
pub struct Range<T> {
    pub min:T,
    pub max:T,
}

impl Range<usize> {
    pub(crate) fn size(&self) -> usize {
        self.max-self.min
    }
}

impl Range<i32> {
    pub(crate) fn is_in_fifty_range(&self) -> bool {
        self.min >= -50 && self.max <= 50
    }
}


impl <T> PartialEq for Range<T> where T:PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl <T> Range<T> where T:Copy {

    // pub fn overlaping(&self, other:&Range) -> Overlaping {
    //     let new_min = self.min.max(other.min);
    //     let new_max = self.max.min(other.max);
    //
    //     if new_min < new_max {
    //         Overlaping::None
    //     } else if new_min == self.min && new_max == other.max {
    //         Overlaping::MinOutside
    //     } else if new_min == self.min && new_max == self.max {
    //         Overlaping::Containing
    //     } else if new_min == other.min && new_max == self.max {
    //         Overlaping::MaxOutside
    //     } else {
    //         Overlaping::Inside
    //     }
    // }
    //
    pub fn range(&self) -> std::ops::Range<T> {
        self.min..self.max
    }

    pub fn of(min:T, max:T) -> Self {
        Range{min,max}
    }

}

impl FromStr for Range<i32> {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let coordinates:Vec<i32> = line[2..].split('.')
            .filter(|s| !s.is_empty())
            .map(|s| parse_input!(s, i32))
            .collect();

        Ok(Range::of(coordinates[0], coordinates[1]+1))
    }
}

