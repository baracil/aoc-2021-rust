use crate::{parse_input, Part};
use crate::days::day_09::map::Map;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day09_launch(part: Part) -> AOCResult<String> {
    let map = parse_input(false)?;
    match part {
        Part::Part1 => part1(&map),
        Part::Part2 => part2(&map)
    }
}

fn part1(map:&Map) -> AOCResult<String> {
    Ok(map.compute_part1().to_string())
}

fn part2(map:&Map) -> AOCResult<String> {
    Ok(map.compute_part2().to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Map> {
    let lines = Problem::factory(for_test)(9).read_input_as_vec_of_line()?;
    Ok(Map::parse(lines))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_09::main::{parse_input, part1, part2};

    #[test]
    fn day09_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result,"15")
    }

    #[test]
    fn day09_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"1134")
    }
}