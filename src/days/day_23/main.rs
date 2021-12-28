
use crate::Part;
use crate::days::day_23::burrow::Burrow;
use crate::days::day_23::organizer::Organizer;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day23_launch(part: Part) -> AOCResult<String> {
    let lines = parse_input(false)?;
    match part {
        Part::Part1 => part1(&lines),
        Part::Part2 => part2(&lines)
    }
}

//18195 to low
fn part1(burrow_map:&[String]) -> AOCResult<String> {
    let burrow = Burrow::for_part1(burrow_map);
    Organizer::organize(&burrow).map(|s| s.to_string()).ok_or_else(|| "Cannot solve day23 part 1".to_string())
}

fn part2(burrow_map:&[String]) -> AOCResult<String> {
    let burrow = Burrow::for_part2(burrow_map);
    Organizer::organize(&burrow).map(|s| s.to_string()).ok_or_else(|| "Cannot solve day23 part 2".to_string())
}



#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<Vec<String>> {
    Problem::factory(for_test)(23).read_input_as_vec_of_line()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_23::burrow::Burrow;
    use crate::days::day_23::main::{parse_input, part1, part2};

    #[test]
    fn day32_test_allowed_displacements_01() {
        let map = ["#############","#...........#","###B#C#B#D###","  #A#D#C#A#  ","  #########  "].map(|s| s.to_string());
        let burrow = Burrow::for_part1(&map);
        let vec = burrow.allowed_displacements();
        assert_eq!(vec.len(),28)
    }

    #[test]
    fn day32_test_allowed_displacements_02() {
        let map = ["#############","#..B........#","###B#C#.#D###","  #A#D#C#A#  ","  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        assert_eq!(vec.len(),1)
    }

    #[test]
    fn day23_test_allowed_displacements_03() {
        let map = [
            "#############",
            "#...B.......#",
            "###B#.#C#D###",
            "  #A#D#C#A#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),10)
    }

    #[test]
    fn day23_test_allowed_displacements_04() {
        let map = [
            "#############",
            "#...B.D.....#",
            "###B#.#C#D###",
            "  #A#.#C#A#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),1)
    }


    #[test]
    fn day23_test_allowed_displacements_05() {
        let map = [
            "#############",
            "#.....D.....#",
            "###B#.#C#D###",
            "  #A#B#C#A#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),1)
    }

    #[test]
    fn day23_test_allowed_displacements_06() {
        let map = [
            "#############",
            "#.....D.....#",
            "###.#B#C#D###",
            "  #A#B#C#A#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),3)
    }

    #[test]
    fn day23_test_allowed_displacements_07() {
        let map = [
            "#############",
            "#.....D.D...#",
            "###.#B#C#.###",
            "  #A#B#C#A#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),2)
    }

    #[test]
    fn day23_test_allowed_displacements_08() {
        let map = [
            "#############",
            "#.....D.D.A.#",
            "###.#B#C#.###",
            "  #A#B#C#.#  ",
            "  #########  "].map(|s| s.to_string());
        let burrow = Burrow::parse_map(&map,2);
        let vec = burrow.allowed_displacements();
        println!("{:?}",vec);
        assert_eq!(vec.len(),1)
    }

    #[test]
    fn day23_part1_test()  {
        let lines = parse_input(true).unwrap();
        let result = part1(&lines).unwrap();
        assert_eq!(result,"12521")
    }

    #[test]
    fn day23_part2_test()  {
        let lines = parse_input(true).unwrap();
        let result = part2(&lines).unwrap();
        assert_eq!(result,"44169")
    }
}