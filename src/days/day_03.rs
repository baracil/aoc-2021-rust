use crate::Part;
use crate::problem::{Problem, Result};

enum BalanceResult {
    Value(i32),
    Balance(i32)
}


struct DiagnosticReport {
    size: usize,
    values: Vec<i32>,
}

impl DiagnosticReport {
    fn compute_power_consumption(&self) -> PowerConsumption {
        let mut gamma = 0;

        for i in 0..self.size {
            let mask = 1 << i;


            let balance = self.values.iter().fold(0, |count,value| count + get_bit_offset(value, mask));
            if balance > 0 {
                gamma += mask;
            }
        };
        let epsilon = ((1 << self.size) - 1) ^ gamma;
        return PowerConsumption { gamma_rate: gamma, epsilon_rate: epsilon };
    }



    fn compute_life_support(&self) -> LifeSupport {
        let oxygen_generator_rating = self.compute_life_support_rating(|b| b>=0);
        let co2_scrubber_rating = self.compute_life_support_rating(|b| b<0);

        return LifeSupport{oxygen_generator_rating,co2_scrubber_rating}

    }

    fn compute_life_support_rating(&self, criteria_predicate: impl Fn(i32) -> bool) -> i32 {
        let mut sieve_value = 0;
        let mut sieve_mask = 0;
        for bit_pos in (0..self.size).rev() {

            let mask = 1 << bit_pos;
            let balance_result = compute_bit_balance(&self.values, sieve_value, sieve_mask, mask);


            match balance_result {
                BalanceResult::Value(rating) => return rating,
                BalanceResult::Balance(balance) =>  {
                    let criteria_matches = criteria_predicate(balance);

                    sieve_mask |= mask;
                    if criteria_matches {
                        sieve_value |= mask
                    }
                }
            }
        }
        return sieve_value
    }
}


fn get_bit_offset(value:&i32, bit_mask:i32) -> i32 {
    if (value & bit_mask) == 0 { -1 } else {  1 }
}

fn compute_bit_balance(values: &Vec<i32>, sieve_value: i32, sieve_mask: i32, bit_mask: i32) -> BalanceResult {
    let mut balance = 0;
    let mut nb_matches = 0;
    let mut a_value = -1;
    for value in values {

        let value_match = (*value & sieve_mask) == sieve_value;

        if !value_match {
            continue
        }

        nb_matches += 1;
        a_value = *value;
        balance += get_bit_offset(value,bit_mask);
    };

    match nb_matches {
        1 => BalanceResult::Value(a_value),
        _ => BalanceResult::Balance(balance)
    }

}

struct PowerConsumption {
    gamma_rate: i32,
    epsilon_rate: i32,
}

struct LifeSupport {
    oxygen_generator_rating: i32,
    co2_scrubber_rating: i32,
}


#[allow(dead_code)]
pub fn day03_launch(part: Part) -> Result<String> {
    let diagnostics = parse_input(false)?;
    match part {
        Part::Part1 => Ok(part1(&diagnostics).to_string()),
        Part::Part2 => Ok(part2(&diagnostics).to_string())
    }
}

fn part1(diagnostics: &DiagnosticReport) -> i32 {
    let rate = diagnostics.compute_power_consumption();
    rate.gamma_rate * rate.epsilon_rate
}

fn part2(diagnostics: &DiagnosticReport) -> i32 {
    let rate = diagnostics.compute_life_support();
    rate.oxygen_generator_rating*rate.co2_scrubber_rating
}


fn parse_input(for_test: bool) -> Result<DiagnosticReport> {
    let lines = Problem::factory(for_test)(3).read_input_as_vec_of_line()?;
    let size = lines.get(0).map_or(0, |l| l.len());

    if size == 0 {
        return Err("No data".to_string());
    }

    let values = lines.iter().map(|l| i32::from_str_radix(l, 2).unwrap()).collect();
    Ok(DiagnosticReport { size, values })
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_03::{parse_input, part1, part2};

    #[test]
    fn day03_part1_test() {
        let diagnostic = parse_input(true).unwrap();
        let result = part1(&diagnostic);
        assert_eq!(result, 198)
    }

    #[test]
    fn day03_part2_test() {
        let diagnostic = parse_input(true).unwrap();
        let result = part2(&diagnostic);
        assert_eq!(result, 230)
    }
}