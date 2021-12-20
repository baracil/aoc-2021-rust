use crate::day_enum::Day;
use crate::days::day_18::main::day18_launch;
use crate::days::day_19::main::day19_launch;
use crate::problem::{AOCResult, Part};

mod problem;
mod domain;
mod days;
mod macros;
pub mod tools;
mod day_enum;


fn main() {
    solve(Day::D20);
}


fn solve(day:Day) {
    println!("{:?} part 1 : {}",day, day.solver()(Part::Part1).unwrap_or_else(|e|  e));
    println!("{:?} part 2 : {}",day, day.solver()(Part::Part2).unwrap_or_else(|e|  e));
}
