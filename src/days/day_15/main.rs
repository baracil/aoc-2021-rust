use crate::Part;
use crate::days::day_15::map::Map;
use crate::days::day_15::path_finder::find_lowest_risk;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day15_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines: &[String]) -> AOCResult<String> {
    let map = Map::parse_part1(lines);
    Ok(find_lowest_risk(&map).to_string())
}

fn part2(lines: &[String]) -> AOCResult<String> {
    let map = Map::parse_part2(lines);
    Ok(find_lowest_risk(&map).to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(15).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_15::main::{parse_input, part1, part2};

    #[test]
    fn day15_part1_test() {
        let map = parse_input(true).unwrap();
        let result = part1(&map).unwrap();
        assert_eq!(result, "40")
    }

    #[test]
    fn day15_part2_test() {
        let map = parse_input(true).unwrap();
        let result = part2(&map).unwrap();
        assert_eq!(result, "315")
    }
}