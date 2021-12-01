use crate::Part;
use crate::problem::{Problem, Result};

pub fn day02_launch(part: Part) -> Result<String> {
    let problem = Problem::new(1);
    match part {
        Part::Part1 => day02_part1(&problem),
        Part::Part2 => day02_part2(&problem)
    }
}

fn day02_part1(problem: &Problem) -> Result<String> {
    Err("Not Solved Yet".to_string())
}

fn day02_part2(problem: &Problem) -> Result<String> {
    Err("Not Solved Yet".to_string())
}

