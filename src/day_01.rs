use std::io;

use crate::{to_vec_of_int};
use crate::problem::{Part, Problem, Result};

pub fn day01_launch(part: Part) -> Result<String> {
    let problem = Problem::new(1);
    match part {
        Part::Part1 => day01_part1(&problem),
        Part::Part2 => day01_part2(&problem)
    }
}

fn day01_part1(problem: &Problem) -> Result<String> {
    problem.read_as_vec_of_u32()
        .map(|d| count_increase(d, 1))
}

fn day01_part2(problem: &Problem) -> Result<String> {
    problem.read_as_vec_of_u32()
        .map(|d| count_increase(d, 3))
}

fn count_increase(depths: Vec<u32>, step: usize) -> String {
    let mut count: u32 = 0;
    for pos in 0..(depths.len() - step) {
        if depths[pos + step] > depths[pos] {
            count += 1
        }
    }
    format!("{}", count)
}