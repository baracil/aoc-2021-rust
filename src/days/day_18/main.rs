use crate::days::day_18::number::Number;
use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day18_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

fn part1(lines: &[String]) -> AOCResult<String> {
    let number = lines.iter()
        .map(|l| parse_input!(l,Number))
        .reduce(|n1, n2| n1 + n2)
        .ok_or_else(|| "No number to add".to_string())?;

    Ok(number.magnitude().to_string())
}

fn part2(lines: &[String]) -> AOCResult<String> {
    let numbers: Vec<Number> = lines.iter()
        .map(|l| parse_input!(l,Number)).collect();

    let nb_numbers = numbers.len();

    (0..nb_numbers - 1)
        .flat_map(move |i| (i + 1..nb_numbers).map(move |j| (i, j)))
        .map(|(i, j)| compute_max_magnitude(&numbers[i].clone(), &numbers[j].clone()))
        .max()
        .map(|m| m.to_string())
        .ok_or_else(|| "No number to work with".to_string())
}

fn compute_max_magnitude(n1: &Number, n2: &Number) -> usize {
    let sum1 = n1.clone() + n2.clone();
    let sum2 = n2.clone() + n1.clone();

    sum1.magnitude().max(sum2.magnitude())
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(18).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_18::main::{parse_input, part1, part2};
    use crate::days::day_18::number::Number;

    #[test]
    fn day18_parsing_test() {
        let number = parse_input!("[[[[[9,8],1],2],3],4]",Number);
        assert_eq!(number.to_string(), "[[[[[9,8],1],2],3],4]");
    }

    #[test]
    fn day18_test_explode_01() {
        let mut number = parse_input!("[[[[[9,8],1],2],3],4]",Number);
        number.explode();
        println!("{}", number);
        assert_eq!(number.to_string(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn day18_test_explode_02() {
        let mut number = parse_input!("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",Number);
        number.explode();
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn day18_test_explode_03() {
        let mut number = parse_input!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",Number);
        number.explode();
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn day18_test_explode_04() {
        let mut number = parse_input!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",Number);
        number.explode();
        assert_eq!(number.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    }

    #[test]
    fn day18_test_explode_05() {
        let mut number = parse_input!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",Number);
        number.explode();
        assert_eq!(number.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
    }


    #[test]
    fn day18_test_explode_06() {
        let mut number = parse_input!("[7,[6,[5,[4,[3,2]]]]]",Number);
        number.explode();
        println!("{}", number);
        assert_eq!(number.to_string(), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn day18_test_explode_07() {
        let mut number = parse_input!("[[6,[5,[4,[3,2]]]],1]",Number);
        number.explode();
        println!("{}", number);
        assert_eq!(number.to_string(), "[[6,[5,[7,0]]],3]");
    }


    #[test]
    fn day18_test_split_01() {
        let mut number = parse_input!("[[[[0,7],4],[15,[0,13]]],[1,1]]",Number);
        number.split();
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
    }

    #[test]
    fn day18_test_split_02() {
        let mut number = parse_input!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",Number);
        number.split();
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    }

    #[test]
    fn day18_test_reduce() {
        let mut number = parse_input!("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",Number);
        number.reduce();
        assert_eq!(number.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn day18_test_add_01() {
        let number1 = parse_input!("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",Number);
        let number2 = parse_input!("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",Number);
        let sum = number1 + number2;
        assert_eq!(sum.to_string(), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }

    #[test]
    fn day18_test_add_02() {
        let number1 = parse_input!("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",Number);
        let number2 = parse_input!("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",Number);
        let sum = number1 + number2;
        assert_eq!(sum.to_string(), "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
    }

    #[test]
    fn day18_test_add_03() {
        let number1 = parse_input!("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",Number);
        let number2 = parse_input!("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",Number);
        let sum = number1 + number2;
        assert_eq!(sum.to_string(), "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");
    }

    #[test]
    fn day18_part1_test_01() {
        let lines = parse_input(true).unwrap();
        let result = part1(&lines).unwrap();
        assert_eq!(result, "4140")
    }

    #[test]
    fn day18_part1_test_02() {
        let lines: Vec<String> = ["[1,1]", "[2,2]", "[3,3]", "[4,4]"].iter().map(|s| s.to_string()).collect();
        let n = lines.iter().map(|l| parse_input!(l,Number)).reduce(|n1, n2| n1 + n2).unwrap();
        assert_eq!(n.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn day18_part1_test_03() {
        let lines: Vec<String> = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"].iter().map(|s| s.to_string()).collect();
        let n = lines.iter().map(|l| parse_input!(l,Number)).reduce(|n1, n2| n1 + n2).unwrap();
        assert_eq!(n.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }


    #[test]
    fn day18_part2_test() {
        let _input = parse_input(true).unwrap();
        let result = part2(&_input).unwrap();
        assert_eq!(result, "3993")
    }
}