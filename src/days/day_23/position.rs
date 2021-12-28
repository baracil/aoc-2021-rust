use std::fmt::{Display, Formatter};
use crate::days::day_23::consts::{HALLWAY_Y, STRIDE};

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub(crate) fn distance(&self, other: &Position) -> usize {
        ((self.y - HALLWAY_Y) + (other.y - HALLWAY_Y) + (self.x-other.x).abs()) as usize
    }
}


impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

impl Position {
    pub(crate) fn from_linear_coordinate(coordinate: usize) -> Position {
        let x = coordinate % STRIDE;
        let y = coordinate / STRIDE;
        Position{x:x as i32, y:y as i32}
    }

    pub fn of(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    // pub fn manhattan(&self, other: &Position) -> usize {
    //     ((other.x - self.x).abs() + (other.y - self.y).abs()) as usize
    // }
    //
    pub fn linear_coordinate(&self) -> usize {
        (self.x as usize + self.y as usize * STRIDE) as usize
    }

    // pub fn not_in_front_of_rooms(&self) -> bool {
    //     ROOM_X.iter().all(|x| self.x != *x)
    // }
}