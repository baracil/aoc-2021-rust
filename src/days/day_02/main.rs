use crate::domain::{SubCommand, Submarine};
use crate::Part;
use crate::problem::{Problem, AOCResult};

#[allow(dead_code)]
pub fn day02_launch(part: Part) -> AOCResult<String> {
    let commands = parse_input(false)?;
    let result = match part {
        Part::Part1 => day02_part1(&commands),
        Part::Part2 => day02_part2(&commands)
    };
    Ok(result.to_string())
}

fn day02_part1(commands: &[SubCommand]) -> i32 {
    let submarine:Submarine = commands.iter().fold(Submarine::default(),|s,c| s.apply_command_stupid(c));
    submarine.horizontal()*submarine.depth()
}

fn day02_part2(commands: &[SubCommand]) -> i32 {
    let submarine:Submarine = commands.iter().fold(Submarine::default(),|s,c| s.apply_command(c));
    submarine.horizontal()*submarine.depth()
}

fn parse_input(for_test:bool) -> AOCResult<Vec<SubCommand>> {
    Problem::factory(for_test)(2).read_input_as_mapped_lines(|line| line.parse::<SubCommand>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::days::day_02::main::{day02_part1, day02_part2, parse_input};

    #[test]
    fn day02_part1_test()  {
        let commands = parse_input(true).unwrap();
        let result = day02_part1(&commands);
        assert_eq!(result,150)
    }

    #[test]
    fn day02_part2_test()  {
        let commands = parse_input(true).unwrap();
        let result = day02_part2(&commands);
        assert_eq!(result,900)
    }
}