use std::ops::Sub;
use crate::domain::{SubCommand, Submarine, Position};
use crate::Part;
use crate::problem::{Problem, Result};

pub fn day02_launch(part: Part) -> Result<String> {
    let problem = Problem::new(2);
    let commands = parse_commands(&problem)?;
    match part {
        Part::Part1 => day02_part1(&commands),
        Part::Part2 => day02_part2(&commands)
    }
}

fn day02_part1(commands: &Vec<SubCommand>) -> Result<String> {
    let mut submarine:Submarine = Submarine::default();

    for command in commands {
        submarine.apply_command_stupid(command);
    }

    Ok((submarine.horizontal()*submarine.depth()).to_string())
}

fn day02_part2(commands: &Vec<SubCommand>) -> Result<String> {
    let mut submarine:Submarine = Submarine::default();

    for command in commands {
        submarine.apply_command(command);
    }

    Ok((submarine.horizontal()*submarine.depth()).to_string())
}


fn parse_commands(problem:&Problem) -> Result<Vec<SubCommand>> {
    problem.read_input_map_line(|line| line.parse::<SubCommand>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::day_02::{day02_part1, day02_part2, parse_commands};
    use crate::problem::Problem;

    #[test]
    fn day02_part1_test()  {
        let problem = Problem::new_test(2);
        let commands = parse_commands(&problem).unwrap();
        let result = day02_part1(&commands).unwrap();
        assert_eq!(result,"150")
    }

    #[test]
    fn day02_part2_test()  {
        let problem = Problem::new_test(2);
        let commands = parse_commands(&problem).unwrap();
        let result = day02_part2(&commands).unwrap();
        assert_eq!(result,"900")
    }
}