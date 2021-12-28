
use crate::days::day_23::consts::{CHAR, ENERGY, HALLWAY_Y, ROOM_X};
use crate::days::day_23::position::Position;

#[derive(Debug, Clone)]
pub struct Amphipod {
    id: usize,
    family: usize,
    position: Position,
}



impl Amphipod {
    pub fn right_x(&self) -> bool {
        self.position.x == ROOM_X[self.family]
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn char(&self) -> char {
        CHAR[self.family]
    }

    pub fn required_energy(&self) -> usize {
        ENERGY[self.family]
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn y(&self) -> i32 {
        self.position.y
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn with_position(&self, position: Position) -> Self {
        Amphipod { id: self.id, position, family: self.family }
    }

    pub fn from(id: usize, family: u8, position: Position) -> Self {
        Amphipod { id, family: family as usize, position }
    }

    pub fn room_x(&self) -> i32 {
        ROOM_X[self.family]
    }

    pub fn is_in_hallway(&self) -> bool {
        self.position.y == HALLWAY_Y
    }
}





/*
 01234567901
#############
#...........#0
###A#C#B#A###1
  #D#D#B#C#  2
  #########

 */