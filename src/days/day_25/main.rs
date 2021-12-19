use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day25_launch(part: Part) -> AOCResult<String> {
    match part {
        Part::Part1 => part1(),
        Part::Part2 => part2()
    }
}

fn part1() -> AOCResult<String> {
    Err("Not Solved Yet".to_string())
}

fn part2() -> AOCResult<String> {
    Err("Not Solved Yet".to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<String> {
    Problem::factory(for_test)(25).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_25::main::{parse_input, part1, part2};

    #[test]
    #[ignore]
    fn day25_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1().unwrap();
        assert_eq!(result,"")
    }

    #[test]
    #[ignore]
    fn day25_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2().unwrap();
        assert_eq!(result,"")
    }
}