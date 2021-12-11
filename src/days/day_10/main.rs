use crate::days::day_10::complete::complete_score;
use crate::days::day_10::syntax_checker::syntax_score;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day10_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines:&Vec<String>) -> AOCResult<String> {
    let score:usize = lines.iter()
        .map(|l| syntax_score(l))
        .sum();

    Ok(score.to_string())

}

fn part2(lines:&Vec<String>) -> AOCResult<String> {
    let mut score:Vec<usize> = lines.iter().flat_map(|l| complete_score(l)).collect();
    score.sort();
    let len = score.len();

    Ok(score[len / 2].to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(10).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_10::main::{parse_input, part1, part2};

    #[test]
    fn day10_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result,"26397")
    }

    #[test]
    fn day10_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"288957")
    }
}