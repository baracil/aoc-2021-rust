use crate::days::day_25::sea_floor::SeaFloor;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day25_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines:&[String]) -> AOCResult<String> {
    let mut sea_floor = SeaFloor::parse(lines)?;
    (1+1..).take_while(|_| sea_floor.perform_one_step())
        .last()
        .map(|i| i.to_string())
        .ok_or_else(|| "Cannot solve this...".to_string())
}

fn part2(lines:&[String]) -> AOCResult<String> {
    let _sea_floor = SeaFloor::parse(lines)?;
    todo!()
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(25).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_25::main::{parse_input, part1, part2};
    use crate::days::day_25::sea_floor::SeaFloor;

    #[test]
    fn day25_part1_test()  {
        let lines = parse_input(true).unwrap();
        let result = part1(&lines).unwrap();
        assert_eq!(result,"58")
    }

    #[test]
    #[ignore]
    fn day25_check_step_generation()  {
        let lines = parse_input(true).unwrap();
        let mut sea_floor = SeaFloor::parse(&lines).unwrap();

        println!("{}",sea_floor);
        sea_floor.perform_one_step();
        println!("{}",sea_floor);


    }

    #[test]
    #[ignore]
    fn day25_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"")
    }
}