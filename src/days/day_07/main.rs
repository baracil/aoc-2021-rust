use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day07_launch(part: Part) -> AOCResult<String> {
    let crabs = parse_input(false)?;
    match part {
        Part::Part1 => part1(&crabs),
        Part::Part2 => part2(&crabs)
    }
}

fn part1(crabs: &[i32]) -> AOCResult<String> {
    let size = crabs.len();
    let consumption = move |target: i32| -> i32 { crabs.iter().map(|i| (*i - target).abs()).sum() };
    let middle = crabs.get(size.div_euclid(2)).ok_or_else(|| "???".to_string())?;
    Ok(consumption(*middle).to_string())
}

fn part2(crabs: &[i32]) -> AOCResult<String> {
    let (p_min,p_max) = find_minimum_position(crabs);
    let consumption = move |target: i32| -> i32 { crabs.iter().map(|i|arithmetic_consumption(*i,target)).sum() };

    let result = {
        let v_min = consumption(p_min);
        if p_min == p_max {
            v_min
        } else {
            let v_max = consumption(p_max);
            v_max.min(v_min)
        }
    };
    Ok((result).to_string())
}

fn find_minimum_position(crabs: &[i32]) -> (i32,i32) {
    let derivate = |target:i32| -> i32 {crabs.iter().map(|crab| derivate_part2(*crab,target)).sum()};

    let mut p_min = *crabs.get(0).unwrap();
    let mut p_max = *crabs.get(crabs.len() - 1).unwrap();
    let mut v_min = derivate(p_min);
    let mut v_max = derivate(p_max);

    while p_max>(p_min+1) {
        let p_mid = (p_max+p_min).div_euclid(2);
        let v_mid = derivate(p_mid);

        if v_mid == 0 {
            return (p_mid,p_mid);
        }

        if v_mid.signum() == v_max.signum() {
            p_max = p_mid;
            v_max = v_mid;
        } else {
            p_min = p_mid;
            v_min = v_mid;
        }
    };

    if v_min == 0 {
        (p_min,p_min)
    } else if v_max == 0 {
        (p_max,p_max)
    } else {
        (p_min,p_max)
    }
}

fn arithmetic_consumption(position: i32, target: i32) -> i32 {
    let dif = (position - target).abs();
    (dif * (dif + 1)).div_euclid(2)
}

fn derivate_part2(position:i32, target:i32) -> i32 {
    let dif = position - target;
    (2*dif.abs()+1)*dif.signum()
}


#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<i32>> {
    let mut crabs: Vec<i32> = Problem::factory(for_test)(7)
        .read_input()?.split(',')
        .map(|s| parse_input!(s,i32))
        .collect();
    crabs.sort_unstable();
    Ok(crabs)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_07::main::{parse_input, part1, part2};

    #[test]
    fn day07_part1_test() {
        let crabs = parse_input(true).unwrap();
        let result = part1(&crabs).unwrap();
        assert_eq!(result, "37")
    }

    #[test]
    fn day07_part2_test() {
        let crabs = parse_input(true).unwrap();
        let result = part2(&crabs).unwrap();
        assert_eq!(result, "168")
    }
}