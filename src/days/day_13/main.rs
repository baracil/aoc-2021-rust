use crate::days::day_13::fold::Fold;
use crate::days::day_13::sheet::Sheet;
use crate::{parse_input, Part};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day13_launch(part: Part) -> AOCResult<String> {
    let (sheet, folds) = parse_input(false)?;
    match part {
        Part::Part1 => part1(&sheet, &folds),
        Part::Part2 => part2(&sheet, &folds)
    }
}

fn part1(sheet:&Sheet, folds:&[Fold]) -> AOCResult<String> {
    let one_fold_sheet = sheet.fold(&folds[0]);

    Ok(one_fold_sheet.number_of_dots().to_string())
}

fn part2(sheet:&Sheet, folds:&[Fold]) -> AOCResult<String> {
    let fully_folded = folds.iter().fold(sheet.clone(), |s, f| s.fold(f));
    Ok(fully_folded.to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<(Sheet,Vec<Fold>)> {
    let input = Problem::factory(for_test)(13).read_input()?;

    let option = input.split_once("\n\n").ok_or("Cannot parse input")?;

    let sheet = Sheet::parse(option.0);

    let result:Vec<Fold> = option.1.split('\n').map(|l| parse_input!(l,Fold)).collect();

    Ok((sheet, result))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_13::main::{parse_input, part1, part2};

    #[test]
    fn day13_part1_test()  {
        let (sheet,folds) = parse_input(true).unwrap();
        let result = part1(&sheet, &folds).unwrap();
        assert_eq!(result,"17")
    }

    #[test]
    fn day13_part2_test()  {
        let (sheet,folds) = parse_input(true).unwrap();
        let result = part2(&sheet, &folds).unwrap();
        assert_eq!(result,"\n█████
█   █
█   █
█   █
█████")
    }
}