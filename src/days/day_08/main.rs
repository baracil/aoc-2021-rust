use crate::days::day_08::line::Line;
use crate::Part;
use crate::problem::{AOCResult, Problem};

///
/// for each digit lets write a array of size 7 with
///   1 if the segment is used to display it
///   0 if the segment is not used
///
///      a b c d e f g
/// 0 :  1 1 1 0 1 1 1  (6 segments)
/// 1 :  0 0 1 0 0 1 0  (2
/// 2 :  1 0 1 1 1 0 1  (5
/// 3 :  1 0 1 1 0 1 1  (5
/// 4 :  0 1 1 1 0 1 0  (4
/// 5 :  1 1 0 1 0 1 1  (5
/// 6 :  1 1 0 1 1 1 1  (6
/// 7 :  1 0 1 0 0 1 0  (3
/// 8 :  1 1 1 1 1 1 1  (7
/// 9 :  1 1 1 1 0 1 1  (6
///
/// In the problem columns a to g are permuted.
/// The other information available is the number of segments
/// used to display the digit. So lets group the digits by
/// number of segments and add the array
///
///             a b c d e f g
/// 0,6,9 a_6 = 3 3 2 2 2 3 3  (6 segments)
///
/// 2,3,5 a_5 = 3 1 2 3 1 2 3  (5
///
/// 7     a_3 = 1 0 1 0 0 1 0  (3
///
/// The other are not used so not display here.
///
/// If you compute (a_6+a_5-4xa_3) you get
///                a b c d e f g
/// a_6+a_5-4*a_3  2 4 0 5 3 1 6
///
/// So if you perform the same operation with permutated letter
/// you can match with the histogram wich letter correspond to (the
/// one with 0 in the result is c, the one with 1 is f ...
///
#[allow(dead_code)]
pub fn day08_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;

    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines: &Vec<Line<String>>) -> AOCResult<String> {

    Ok(lines.iter()
        .flat_map(|l| l.digits().iter())
        .filter(|l| match l.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        })
        .count().to_string())
}

fn part2(lines: &Vec<Line<String>>) -> AOCResult<String> {
    let lines: Vec<Line<u32>> = lines
        .iter()
        .map(Line::transform_to_u32)
        .collect();

    let result:u32 = lines.iter().map(|l| l.display_value())
        .sum();

    Ok(result.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<Line<String>>> {
    Problem::factory(for_test)(8)
        .read_input_as_mapped_lines(|s| s.parse::<Line<String>>().unwrap())
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_08::main::{parse_input, part1, part2};

    #[test]
    fn day08_part1_test() {
        let _input = parse_input(true).unwrap();
        _input.iter().for_each(|l| println!("{}", l));
        let result = part1(&_input).unwrap();
        assert_eq!(result, "26")
    }

    #[test]
    fn day08_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "61229")
    }
}