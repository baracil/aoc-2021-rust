use crate::days::day_02::day02_launch;
use crate::days::day_03::day03_launch;
use crate::problem::Part;

mod problem;
mod domain;
mod days;


#[warn(dead_code)]
fn main() {
    let launch = day03_launch;

    println!("part 1 : {:?}",launch(Part::Part1));
    println!("part 2 : {:?}",launch(Part::Part2));
}
