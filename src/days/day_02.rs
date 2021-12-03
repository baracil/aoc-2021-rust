use crate::domain::{SubCommand, Submarine};
use crate::Part;
use crate::problem::{Problem, Result};

pub fn day02_launch(part: Part) -> Result<String> {
    let commands = parse_commands(false)?;
    let result = match part {
        Part::Part1 => day02_part1(&commands),
        Part::Part2 => day02_part2(&commands)
    };
    Ok(result.to_string())
}

fn day02_part1(commands: &Vec<SubCommand>) -> i32 {
    let mut submarine:Submarine = Submarine::default();

    for command in commands {
        submarine.apply_command_stupid(command);
    }

    submarine.horizontal()*submarine.depth()
}

fn day02_part2(commands: &Vec<SubCommand>) -> i32 {
    let mut submarine:Submarine = Submarine::default();

    for command in commands {
        submarine.apply_command(command);
    }

    submarine.horizontal()*submarine.depth()
}

fn parse_commands(for_test:bool) -> Result<Vec<SubCommand>> {
    Problem::factory(for_test)(2).read_input_as_mapped_lines(|line| line.parse::<SubCommand>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::days::day_02::{day02_part1, day02_part2, parse_commands};

    #[test]
    fn day02_part1_test()  {
        let commands = parse_commands(true).unwrap();
        let result = day02_part1(&commands);
        assert_eq!(result,150)
    }

    #[test]
    fn day02_part2_test()  {
        let commands = parse_commands(true).unwrap();
        let result = day02_part2(&commands);
        assert_eq!(result,900)
    }
}