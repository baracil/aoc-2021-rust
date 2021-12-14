use crate::days::day_06::school::School;
use crate::{Part};
use crate::problem::{AOCResult, Problem};

const GEN_80:[u128;9]=[1421,1401,1191,1154,1034,950,905,779,768];
const GEN_256:[u128;9]=[6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552, 3369186778];

/// The numbers have been computed by starting with 1 fish with timer 0, then one fish with timer 1 ...
///
/// There is an analytic solution but not that practical.
///
/// If you put the fish in a vector of size 9, the next generation can be computed by multiplying with the matrix
///
///     | 010000000 |
///     | 001000000 |
///     | 000100000 |
///     | 000010000 |
/// M = | 000001000 |
///     | 000000100 |
///     | 100000010 |
///     | 000000001 |
///     | 100000000 |
///
/// So gen_n+1 = M.generation_n
///
/// so gen_n = M^n gen_0
///
/// M is diagonalizable (but in Complex space): So P and D (diagonal) exist :
///
/// gen_n = (P.D.P^-1)^n gen_0 => gen_n = P.D^n.P^-1 gen_0
///
/// Computing D^n is then easy since it is diagonal.
///
/// Practically this is not easy because the eigenvalues are the solution of L^9-L-1 = 0 and no analytic solution
/// could be found which means that numerical error might prevent the computation for high n
///
///

#[allow(dead_code)]
pub fn day06_launch(part: Part) -> AOCResult<String> {
    let school = parse_input(false)?;
    match part {
        Part::Part1 => part1(&school),
        Part::Part2 => part2(&school)
    }
}

fn solve(school:&School, nb_generation:u32) -> u128 {
    let mut school = *school;
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