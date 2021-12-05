use crate::days::day_05::vent::Vent;
use crate::{parse_input, Part};
use crate::days::day_05::OverlapCounter::OverlapCounter;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day05_launch(part: Part) -> AOCResult<String> {
    let vents = parse_input(false)?;
    match part {
        Part::Part1 => part1(&vents),
        Part::Part2 => part2(&vents)
    }
}

fn solve(vent:&Vec<Vent>, filter:fn(&Vent)->bool) -> AOCResult<String> {
    let mut counter = OverlapCounter::default();
    let result = vent.iter()
        .filter(|v| filter(v))
        .flat_map(Vent::lines)
        .fold(& mut counter, |a,p| a.push_position(&p));
    Ok(result.nb_overlaps().to_string())

}

fn part1(vent:&Vec<Vent>) -> AOCResult<String> {
    solve(vent,Vent::is_vertical_or_horizontal)
}

fn part2(vent:&Vec<Vent>) -> AOCResult<String> {
    solve(vent,|v| true)
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<Vent>> {
    Problem::factory(for_test)(5).read_input_as_mapped_lines(|line| line.parse::<Vent>().unwrap())
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_05::main::{parse_input, part1, part2};

    #[test]
    fn day05_part1_test()  {
        let vents = parse_input(true).unwrap();
        let result = part1(&vents).unwrap();
        assert_eq!(result,"5")
    }

    #[test]
    fn day05_part2_test()  {
        let vents = parse_input(true).unwrap();
        let result = part2(&vents).unwrap();
        assert_eq!(result,"12")
    }
}