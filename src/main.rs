mod problem;
mod day_01;

use std::fs;
use std::fs::ReadDir;
use crate::day_01::{day01_launch};
use crate::problem::{Part, to_vec_of_int};



fn main() {
    println!("part 1 {:?}",day01_launch(Part::Part1));
    println!("part 2 {:?}",day01_launch(Part::Part2));
}
