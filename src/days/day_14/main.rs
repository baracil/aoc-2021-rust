use std::collections::HashMap;
use crate::days::day_14::computer::DistriComputer;
use crate::Part;
use crate::problem::{AOCResult, Problem};
use crate::tools::{capital_to_couple_u8, capital_to_u8};

#[allow(dead_code)]
pub fn day14_launch(part: Part) -> AOCResult<String> {
    let (template,rules) = parse_input(false)?;
    match part {
        Part::Part1 => part1(&template, &rules),
        Part::Part2 => part2(&template, &rules)
    }
}

fn part1(template:&str, rules:&HashMap<(u8,u8),u8>) -> AOCResult<String> {
    let mut computer = DistriComputer::create(rules);
    Ok(computer.compute_for_template(template,10).to_string())
}

fn part2(template:&str, rules:&HashMap<(u8,u8),u8>) -> AOCResult<String> {
    let mut computer = DistriComputer::create(rules);
    Ok(computer.compute_for_template(template,40).to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<(String, HashMap<(u8,u8),u8>)> {
    let content = Problem::factory(for_test)(14).read_input()?;
    let split_content = content
        .split_once("\n\n")
        .ok_or("Cannot parse input")?;

    let template = split_content.0;
    let rules = split_content.1.split('\n').map(parse_rule)
        .collect::<HashMap<(u8, u8), u8>>();

    Ok((template.to_string(), rules))
}


fn parse_rule(line:&str) -> ((u8,u8),u8) {
    let (couple, insertion) = line.split_once("->")
        .unwrap_or_else(|| panic!("Cannot parse rule {}", line));


    let left = capital_to_u8(couple.chars().next().unwrap());
    let right = capital_to_u8(couple.chars().nth(1).unwrap());

    let insertion = capital_to_u8(insertion.trim().chars().next().unwrap());

    ((left, right),insertion)
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_14::main::{parse_input, part1, part2};

    #[test]
    fn day14_part1_test()  {
        let (template, rules) = parse_input(true).unwrap();
        let result = part1(&template, &rules).unwrap();
        assert_eq!(result,"1588")
    }

    #[test]
    fn day14_part2_test()  {
        let (template, rules) = parse_input(true).unwrap();
        let result = part2(&template, &rules).unwrap();
        assert_eq!(result,"2188189693529")
    }
}