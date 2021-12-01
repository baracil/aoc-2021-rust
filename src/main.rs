mod problem;
mod day_01;
mod day_02;

use std::fs;
use std::fs::ReadDir;
use crate::day_01::{day01_launch};
use crate::day_02::day02_launch;
use crate::problem::{Part, to_vec_of_int};



fn main() {
    let launch = day02_launch;

    println!("part 1 : {:?}",launch(Part::Part1));
    println!("part 2 : {:?}",launch(Part::Part2));
}
