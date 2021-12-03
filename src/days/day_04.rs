use crate::Part;
use crate::problem::{Problem, Result};

#[allow(dead_code)]
pub fn day03_launch(part: Part) -> Result<String> {
    match part {
        Part::Part1 => part1(),
        Part::Part2 => part2()
    }
}

fn part1() -> Result<String> {
    Err("Not Solved Yet".to_string())
}

fn part2() -> Result<String> {
    Err("Not Solved Yet".to_string())
}

fn parse_input(for_test:bool) -> Result<String> {
    Problem::factory(for_test)(3).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_04::{parse_input, part1, part2};
    use crate::problem::Problem;

    #[test]
    #[ignore]
    fn day04_part1_test()  {
        let input = parse_input(true).unwrap();
        let result = part1().unwrap();
        assert_eq!(result,"")
    }

    #[test]
    #[ignore]
    fn day04_part2_test()  {
        let commands = parse_input(true).unwrap();
        let result = part2().unwrap();
        assert_eq!(result,"")
    }
}