
use crate::Part;
use crate::days::day_04::grid_state::GridState::Winning;
use crate::days::day_04::input::Day04Input;
use crate::problem::{Problem, AOCResult};

#[allow(dead_code)]
pub fn day04_launch(part: Part) -> AOCResult<String> {
    let input = parse_input(false)?;
    match part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input)
    }
}

fn part1(input: &Day04Input) -> AOCResult<String> {
    let mut current = input.clone();
    loop {
        let new_input = current.draw_one_number_part1();
        if matches!(new_input,None) {
            return Err("All number has been drawn => no winner".to_string());
        }
        current = new_input.unwrap();
        if let Winning(result) = current.state() {
            return Ok(result.to_string())
        }
    }
}

fn part2(input: &Day04Input) -> AOCResult<String> {
    let mut current = input.clone();
    let mut result:u32 = 0;
    loop {
        let new_input = current.draw_one_number_part2();
        if matches!(new_input,None) {
            return Ok(result.to_string())
        }
        current = new_input.unwrap();
        if let Winning(r) = current.state() {
            result = r
        }
    }

}





#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Day04Input> {
    Problem::factory(for_test)(4).read_input()?.parse::<Day04Input>()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_04::main::{parse_input, part1, part2};

    #[test]
    fn day04_part1_test() {
        let result = parse_input(true).and_then(|i| part1(&i));
        assert_eq!(true, result.is_ok());
        assert_eq!("4512", result.unwrap())
    }

    #[test]
    fn day04_part2_test() {
        let result = parse_input(true).and_then(|i| part2(&i));
        assert_eq!(true, result.is_ok());
        assert_eq!("1924", result.unwrap())
    }
}