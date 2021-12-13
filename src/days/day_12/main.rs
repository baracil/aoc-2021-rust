use crate::days::day_12::caves::Caves;
use crate::days::day_12::path_counter::PathCounter;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day12_launch(part: Part) -> AOCResult<String> {
    let caves = parse_input(false)?;
    match part {
        Part::Part1 => part1(&caves),
        Part::Part2 => part2(&caves)
    }
}

fn part1(caves:&Caves) -> AOCResult<String> {
    Ok(PathCounter::count_pathes_part1(caves).to_string())
}

fn part2(caves:&Caves) -> AOCResult<String> {
    Ok(PathCounter::count_pathes_part2(caves).to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Caves> {
    let input = Problem::factory(for_test)(12).read_input()?;
    Ok(Caves::parse(&input))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_12::caves::Caves;
    use crate::days::day_12::main::{parse_input, part1, part2};

    #[test]
    fn day12_part1_test_01() {
        let caves = parse_input(true).unwrap();
        let result = part1(&caves).unwrap();
        assert_eq!(result, "226")
    }

    #[test]
    fn day12_part1_test_02() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let caves = Caves::parse(input);
        let result = part1(&caves).unwrap();
        assert_eq!(result, "10")
    }

    #[test]
    fn day12_part2_test_01() {
        let caves = parse_input(true).unwrap();
        let result = part2(&caves).unwrap();
        assert_eq!(result, "3509")
    }

    #[test]
    fn day12_part2_test_02() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let caves = Caves::parse(input);
        let result = part2(&caves).unwrap();
        assert_eq!(result, "36")
    }


}