use crate::days::day_08::line::Line;
use crate::Part;
use crate::problem::{AOCResult, Problem};


#[allow(dead_code)]
pub fn day08_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;

    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines: &Vec<Line<String>>) -> AOCResult<String> {

    Ok(lines.iter()
        .flat_map(|l| l.digits().iter())
        .filter(|l| match l.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        })
        .count().to_string())
}

fn part2(lines: &Vec<Line<String>>) -> AOCResult<String> {
    let lines: Vec<Line<u32>> = lines
        .iter()
        .map(Line::transform_to_u32)
        .collect();

    let result:u32 = lines.iter().map(|l| l.display_value())
        .sum();

    Ok(result.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Line<String>>> {
    Problem::factory(for_test)(8)
        .read_input_as_mapped_lines(|s| s.parse::<Line<String>>().unwrap())
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_08::main::{parse_input, part1, part2};

    #[test]
    fn day08_part1_test() {
        let _input = parse_input(true).unwrap();
        _input.iter().for_each(|l| println!("{}", l));
        let result = part1(&_input).unwrap();
        assert_eq!(result, "26")
    }

    #[test]
    fn day08_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "61229")
    }
}