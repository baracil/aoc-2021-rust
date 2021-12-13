use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};
use std::str::FromStr;
use crate::days::day_13::fold::Fold;
use crate::days::day_13::fold::Fold::{X, Y};
use crate::parse_input;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct Dot {
    x: i32,
    y: i32,
}


impl Dot {

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn with(x:i32, y:i32) -> Self {
        Dot{x,y}
    }

    pub fn fold(&self, folding_instruction:&Fold) -> Dot {
        let Dot{x,y} = self;
        match folding_instruction {
            X(xf) => Dot::with(x-2*0.max(x-xf),*y),
            Y(yf) => Dot::with(*x, y-2*0.max(y-yf))
        }
    }
}


impl PartialOrd for Dot {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let yord = self.y.cmp(&other.y);
        Some(yord.then_with(|| self.x.cmp(&other.x)))
    }
}

impl FromStr for Dot {
    type Err = String;

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        let coordinates = point.split_once(",")
            .ok_or_else(|| format!("cannot parse '{}'", point))?;


        let x = parse_input!(coordinates.0,i32);
        let y = parse_input!(coordinates.1,i32);

        Ok(Dot { x, y })
    }
}