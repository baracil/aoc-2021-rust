use crate::days::day_11::map::Map;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day11_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines:&[String]) -> AOCResult<String> {
    let mut map = Map::parse(lines);
    (0..100).for_each(|_| {map.perform_one_step();});
    Ok(map.nb_flashes().to_string())
}

fn part2(lines:&[String]) -> AOCResult<String> {
    let mut map = Map::parse(lines);
    let mut i = 0;
    loop {
        i+=1;
        if map.perform_one_step() {
            return Ok(i.to_string())
        }
    }
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(11).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_11::main::{parse_input, part1, part2};

    #[test]
    #[ignore]
    fn day11_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result,"")
    }

    #[test]
    #[ignore]
    fn day11_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"")
    }
}