use crate::days::day_13::main::day13_launch;
use crate::problem::{AOCResult, Part};

mod problem;
mod domain;
mod days;
mod macros;
pub mod tools;


#[warn(dead_code)]
fn main() {
    let launch = day13_launch;

    println!("part 1 : {}",launch(Part::Part1).unwrap_or_else(|e|  e));
    println!("part 2 : {}",launch(Part::Part2).unwrap_or_else(|e|  e));
}
