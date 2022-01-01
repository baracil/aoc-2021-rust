use crate::days::day_24::model_number::{find_largest_model_number, find_lowest_model_number};
use crate::days::day_24::program::Program;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day24_launch(part: Part) -> AOCResult<String> {
    let program = parse_input(false)?;
    match part {
        Part::Part1 => part1(&program),
        Part::Part2 => part2(&program)
    }
}

fn part1(program:&Program) -> AOCResult<String> {
    find_largest_model_number(program).ok_or_else(|| "Cannot solve this".to_string())
}

fn part2(program:&Program) -> AOCResult<String> {
    find_lowest_model_number(program).ok_or_else(|| "Cannot solve this".to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Program> {
    let lines = Problem::factory(for_test)(24).read_input_as_vec_of_line()?;
    Program::parse(&lines)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

}