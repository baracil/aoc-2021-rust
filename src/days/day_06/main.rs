use crate::days::day_06::school::School;
use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

const GEN_80:[u128;9]=[1421,1401,1191,1154,1034,950,905,779,768];
const GEN_256:[u128;9]=[6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552, 3369186778];

#[allow(dead_code)]
pub fn day06_launch(part: Part) -> AOCResult<String> {
    let school = parse_input(false)?;
    match part {
        Part::Part1 => part1(&school),
        Part::Part2 => part2(&school)
    }
}

fn solve(school:&School, nb_generation:u32) -> u128 {
    let mut school = school.clone();
    for _i in 0..nb_generation {
        school.perform_one_tick();
    }
    school.population_size()
}

fn solve_fast(school:&School, nb_generation:u32) -> AOCResult<String> {
    let result = match nb_generation {
        80 => school.compute_population(&GEN_80),
        256 => school.compute_population(&GEN_256),
        n => solve(school,n)
    };
    Ok(result.to_string())
}

fn part1(school:&School) -> AOCResult<String> {
    solve_fast(school,80)
}

fn part2(school:&School) -> AOCResult<String> {
    solve_fast(school,256)
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<School> {
    Problem::factory(for_test)(6).read_input()?.parse::<School>()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_06::main::{parse_input, part1, part2};

    #[test]
    fn day06_part1_test()  {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result,"5934")
    }

    #[test]
    fn day06_part2_test()  {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result,"26984457539")
    }
}