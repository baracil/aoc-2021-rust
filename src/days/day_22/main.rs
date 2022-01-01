use crate::days::day_22::command::Command;
use crate::{parse_input, Part};
use crate::days::day_22::lights::Lights;
use crate::days::day_22::system::System;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day22_launch(part: Part) -> AOCResult<String> {
    let commands = parse_input(false)?;
    match part {
        Part::Part1 => part1(&commands),
        Part::Part2 => part2(&commands)
    }
}

fn part1(commands: &[Command<i32>]) -> AOCResult<String> {
    let commands:Vec<Command<i32>> = commands.iter().filter(|c| c.is_in_fifty_range()).cloned().collect();
    compute_nb_lights_on(&commands)
}

fn part2(commands: &[Command<i32>]) -> AOCResult<String> {
    compute_nb_lights_on(&commands)
}

fn compute_nb_lights_on(commands: &[Command<i32>]) -> AOCResult<String> {
    let system = System::new(commands);

    let mut lights = Lights::default();

    for command in commands {
        let command = system.transform_command(&command)?;
        lights.apply_command(&command)
    }
    let nb_on = lights.nb_lights_on(&system);

    Ok(nb_on.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Command<i32>>> {
    Problem::factory(for_test)(22).read_input_as_mapped_lines(|line| line.parse::<Command<i32>>().unwrap())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_22::main::{parse_input, part1, part2};

    #[test]
    fn day22_part1_test() {
        let input = parse_input(true).unwrap();
        let result = part1(&input).unwrap();
        assert_eq!(result, "52521")
    }

    #[test]
    #[ignore]
    fn day22_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "")
    }
}