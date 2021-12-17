
use crate::{parse_input, Part};
use crate::days::day_17::data::{Probe, Target};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day17_launch(part: Part) -> AOCResult<String> {
    let target = parse_input(false)?;
    match part {
        Part::Part1 => part1(&target),
        Part::Part2 => part2(&target)
    }
}


fn is_in_target(vx: i32, vy: i32, target: &Target) -> bool {
    let mut probe = Probe::create(vx, vy);
    loop {
        if probe.is_inside(target) {
            break true;
        }
        if probe.missed_target(target) {
            break false;
        }
        probe = probe.apply_one_tick();
    }
}


//3741 to low
fn part1(target: &Target) -> AOCResult<String> {
    let max_vy = (1..-target.ymin()).filter(|vy| (1..target.xmax()).any(|vx| is_in_target(vx,*vy,target))).max().unwrap();
    Ok((max_vy * (max_vy + 1) / 2).to_string())
}

fn part2(target:&Target) -> AOCResult<String> {
    let count = (1..=target.xmax()).flat_map(|x| (target.ymin()-1..-target.ymin()).map(move |y| (x, y)))
        .filter(|(vx, vy)| is_in_target(*vx, *vy, target))
        .count();
    Ok(count.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Target> {
    Problem::factory(for_test)(17).read_input().map(|l| parse_input!(l,Target))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_17::main::{parse_input, part1, part2};

    #[test]
    fn day17_part1_test() {
        let _input = parse_input(true).unwrap();
        let result = part1(&_input).unwrap();
        assert_eq!(result, "45")
    }

    #[test]
    fn day17_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "112")
    }
}