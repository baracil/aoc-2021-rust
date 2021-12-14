use std::str::FromStr;
use crate::days::day_04::grid::{Grid, GRID_SIZE};
use crate::days::day_04::grid_state::GridState;
use crate::days::day_04::grid_state::GridState::{NotWinning, Winning};
use crate::tools::clone_into_array;

#[derive(Clone)]
pub struct Day04Input {
    numbers: Vec<u32>,
    grids: Vec<Grid>,
    state: GridState

}

impl Day04Input {

    pub fn state(&self) -> GridState {
        self.state
    }

    pub fn draw_one_number_part1(&self) -> Option<Day04Input> {
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
            .map(|g| g.state())
            .find(|s| matches!(s,Winning(_)))
            .unwrap_or(NotWinning);

        Some(Day04Input{ numbers, grids, state })
    }

    pub fn draw_one_number_part2(&self) -> Option<Day04Input> {
        if self.numbers.is_empty() {
            return None;
        }
        let mut numbers = self.numbers.clone();
        let drawn_number = numbers.remove(0);

        let grids:Vec<Grid> = self.grids
            .iter()
            .filter(|g| matches!(g.state(),NotWinning))
            .map(|g| g.push_number(drawn_number))
            .collect();

        let state = grids
            .iter()
            .map(|g| g.state())
            .rfind(|s| matches!(s,Winning(_)))
            .unwrap_or(self.state);

        Some(Day04Input{ numbers, grids, state })
    }

}

impl FromStr for Day04Input {
    type Err = String;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = content.lines().collect();

        let numbers: Vec<u32> = lines.get(0)
            .expect("At least one line")
            .split(',')
            .map(|token| token.parse::<u32>().expect("A u32 as string"))
            .collect();

        let empty_lines_index: Vec<usize> = lines.iter()
            .enumerate()
            .filter(|(_idx, line)| line.trim().is_empty())
            .map(|(idx, _line)| idx).collect();

        let grids: Vec<Grid> = empty_lines_index
            .iter()
            .map(|idx| clone_into_array(&lines[idx + 1..idx + 1 + GRID_SIZE]))
            .map(Grid::parse)
            .collect();



        Ok(Day04Input { numbers, grids, state:NotWinning })
    }
}
