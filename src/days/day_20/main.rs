use crate::days::day_20::image::Image;
use crate::days::day_20::rule::Rule;
use crate::{Part, try_parse_input};
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day20_launch(part: Part) -> AOCResult<String> {
    let (image,rule) = parse_input(false)?;
    match part {
        Part::Part1 => part1(&image,&rule),
        Part::Part2 => part2(&image,&rule)
    }
}

fn part1(image:&Image,rule:&Rule) -> AOCResult<String> {
    solve(image,rule,2)
}

fn part2(image:&Image,rule:&Rule) -> AOCResult<String> {
    solve(image,rule,50)
}

fn solve(image:&Image,rule:&Rule, nb_iterations:usize) -> AOCResult<String> {
    let result = (0..nb_iterations).fold(image.clone(),|r,_| r.enhance(rule));

    Ok(result.nb_lit_pixels().to_string())
}


#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<(Image, Rule)> {
    let input = Problem::factory(for_test)(20).read_input()?;
    parse_content(&input,51)
}

fn parse_content(content:&str, border_size:usize) -> AOCResult<(Image, Rule)> {
    let (rule,image) = content.split_once("\n\n").ok_or("Cannot parse input file")?;

    let image = Image::parse(image,border_size)?;
    let rule = try_parse_input!(rule,Rule)?;
    Ok((image, rule))

}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_20::main::{parse_input, part1, part2};

    #[test]
    fn day20_part1_test()  {
        let (image, rule) = parse_input(true).unwrap();
        let result = part1(&image,&rule).unwrap();
        assert_eq!(result,"35")
    }

    #[test]
    fn day20_part2_test()  {
        let (image, rule) = parse_input(true).unwrap();
        let result = part2(&image,&rule).unwrap();
        assert_eq!(result,"3351");
    }
}