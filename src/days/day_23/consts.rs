use lazy_static::lazy_static;
use crate::days::day_23::position::Position;

pub(crate) const ENERGY: [usize; 4] = [1, 10, 100, 1000];
pub(crate) const CHAR: [char; 4] = ['A', 'B', 'C', 'D'];
pub(crate) const ROOM_X: [i32; 4] = [3, 5, 7, 9];
pub(crate) const HALLWAY_Y: i32 = 1;
pub(crate) const STRIDE: usize = 14;
pub(crate) const ROOM_DEPTH_PART1:i32 = 2;
pub(crate) const ROOM_DEPTH_PART2:i32 = 4;


lazy_static! {
pub(crate) static ref ROOM_UP:[Position;4] = [Position::of(3,2),Position::of(5,2),Position::of(7,2),Position::of(9,2)];
pub(crate) static ref ROOM_DOWN:[Position;4] = [Position::of(3,3),Position::of(5,3),Position::of(7,3),Position::of(9,3)];
pub(crate) static ref EXTRA_LINES:[String;2] = ["  #D#C#B#A#  ".to_string(),"  #D#B#A#C#  ".to_string()];
}

pub(crate) fn is_not_in_front_of_a_room(x:i32) -> bool {
    x != 3 && x != 5 && x != 7 && x != 9
}