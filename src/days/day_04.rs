use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::ptr::{null, null_mut};
use std::result::Result as StdResult;
use std::str::FromStr;

use crate::{parse_input, Part};
use crate::days::day_04::GridState::{NotWinning, Winning};
use crate::problem::{clone_into_array, Problem, Result};

#[allow(dead_code)]
pub fn day04_launch(part: Part) -> Result<String> {
    let input = parse_input(false)?;
    match part {
        Part::Part1 => part1(&input),
        Part::Part2 => part2(&input)
    }
}

fn part1(input: &Day04Input) -> Result<String> {
    let mut current = input.clone();
    loop {
        let new_input = current.draw_one_number_part1();
        if matches!(new_input,None) {
            return Err("All number has been drawn => no winner".to_string());
        }
        current = new_input.unwrap();
        if let Winning(result) = current.state {
            return Ok(result.to_string())
        }
    }
}

fn part2(input: &Day04Input) -> Result<String> {
    let mut current = input.clone();
    let mut result:u32 = 0;
    loop {
        let new_input = current.draw_one_number_part2();
        if matches!(new_input,None) {
            return Ok(result.to_string())
        }
        current = new_input.unwrap();
        if let Winning(r) = current.state {
            result = r
        }
    }

}

const GRID_SIZE: usize = 5;
const GRID_SIZE_U32: u32 = 5;

#[derive(Clone)]
struct Grid {
    numbers: HashMap<u32, Position>,
    columns: [u32; GRID_SIZE],
    rows: [u32; GRID_SIZE],
    state:GridState,
}

#[derive(Clone,Copy)]
enum GridState {
    NotWinning,
    Winning(u32),
}

impl Grid {
    fn parse(lines: [&str; GRID_SIZE]) -> Self {
        let mut numbers: HashMap<u32, Position> = lines.iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(move |(column, n)| {
                        (parse_input!(n,u32), Position::of(column , row ))
                    })
            })
            .collect();

        return Grid::new(numbers);
    }

    fn new(numbers: HashMap<u32, Position>) -> Self {
        let columns = [0u32; GRID_SIZE];
        let rows = [0u32; GRID_SIZE];
        Grid { numbers, columns, rows, state:NotWinning }
    }

    fn push_number(&self,number_drawn:u32) -> Self {
        let mut new_grid = self.clone();
        if matches!(self.state,Winning(_)) {
            return new_grid;
        }

        if let Some(position) = new_grid.numbers.remove(&number_drawn) {
            new_grid.columns[position.column]+=1;
            new_grid.rows[position.row]+=1;
            if new_grid.columns[position.column] == GRID_SIZE_U32 || new_grid.rows[position.row] == GRID_SIZE_U32 {
                let result:u32 = new_grid.numbers.keys().map(|u| *u).sum();
                new_grid.state = Winning(result*number_drawn)
            }
        };
        return new_grid;
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    column: usize,
    row: usize,
}

impl Position {
    fn of(column: usize, row: usize) -> Self {
        Position { column, row }
    }
}

#[derive(Clone)]
struct Day04Input {
    numbers: Vec<u32>,
    grids: Vec<Grid>,
    state: GridState

}

impl Day04Input {

    fn draw_one_number_part1(&self) -> Option<Day04Input> {
        if self.numbers.is_empty() {
            return None;
        }
        let mut numbers = self.numbers.clone();
        let drawn_number = numbers.remove(0);

        let grids:Vec<Grid> = self.grids
            .iter()
            .map(|g| g.push_number(drawn_number))
            .collect();

        let state = grids
            .iter()
            .map(|g| g.state)
            .find(|s| matches!(s,Winning(_)))
            .unwrap_or(NotWinning);

        Some(Day04Input{ numbers, grids, state })
    }

    fn draw_one_number_part2(&self) -> Option<Day04Input> {
        if self.numbers.is_empty() {
            return None;
        }
        let mut numbers = self.numbers.clone();
        let drawn_number = numbers.remove(0);

        let mut grids:Vec<Grid> = self.grids
            .iter()
            .filter(|g| matches!(g.state,NotWinning))
            .map(|g| g.push_number(drawn_number))
            .collect();

        let state = grids
            .iter()
            .map(|g| g.state)
            .rfind(|s| matches!(s,Winning(_)))
            .unwrap_or(self.state);

        Some(Day04Input{ numbers, grids, state })
    }

}

impl FromStr for Day04Input {
    type Err = String;

    fn from_str(content: &str) -> StdResult<Self, Self::Err> {
        let lines: Vec<&str> = content.lines().collect();

        let numbers: Vec<u32> = lines.get(0)
            .expect("At least one line")
            .split(",")
            .map(|token| token.parse::<u32>().expect("A u32 as string"))
            .collect();

        let empty_lines_index: Vec<usize> = lines.iter()
            .enumerate()
            .filter(|(idx, line)| line.trim().is_empty())
            .map(|(idx, line)| idx).collect();

        let grids: Vec<Grid> = empty_lines_index
            .iter()
            .map(|idx| clone_into_array(&lines[idx + 1..idx + 1 + GRID_SIZE]))
            .map(|sub_lines| Grid::parse(sub_lines))
            .collect();



        Ok(Day04Input { numbers, grids, state:NotWinning })
    }
}

#[allow(dead_code)]
fn parse_input(for_test: bool) -> Result<Day04Input> {
    Problem::factory(for_test)(4).read_input()?.parse::<Day04Input>()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_04::{parse_input, part1, part2};

    #[test]
    fn day04_part1_test() {
        let result = parse_input(true).and_then(|i| part1(&i));
        assert_eq!(true, result.is_ok());
        assert_eq!("4512", result.unwrap())
    }

    #[test]
    fn day04_part2_test() {
        let result = parse_input(true).and_then(|i| part2(&i));
        assert_eq!(true, result.is_ok());
        assert_eq!("1924", result.unwrap())
    }
}