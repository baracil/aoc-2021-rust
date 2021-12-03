use crate::problem::{Part, Problem, Result};

#[allow(dead_code)]
pub fn day01_launch(part: Part) -> Result<String> {
    let depths = parse_depths(false)?;
    let count = match part {
        Part::Part1 => part1(&depths),
        Part::Part2 => part2(&depths)
    };
    Ok(count.to_string())
}

fn part1(depths: &Vec<u32>) -> u32 {
    count_increasing(depths,1)
}

fn part2(depths: &Vec<u32>) -> u32 {
    count_increasing(depths,3)
}

fn parse_depths(for_test:bool) -> Result<Vec<u32>> {
    let problem = if for_test {Problem::test_of_day(1)} else {Problem::of_day(1)};

    problem.read_input_as_vec_of_u32()
}


fn count_increasing(depths: &Vec<u32>, step: usize) -> u32 {
    let mut count: u32 = 0;
    for pos in 0..(depths.len() - step) {
        if depths[pos + step] > depths[pos] {
            count += 1
        }
    };
    count
}

#[cfg(test)]
mod tests {
    use crate::days::day_01::{part1, part2, parse_depths};

    #[test]
    fn day01_part1_test() {
        let commands = parse_depths(true).unwrap();
        let result = part1(&commands);
        assert_eq!(result, 7)
    }

    #[test]
    fn day01_part2_test() {
        let commands = parse_depths(true).unwrap();
        let result = part2(&commands);
        assert_eq!(result, 5)
    }
}