use std::collections::HashMap;
use crate::days::day_04::grid_state::GridState;
use crate::days::day_04::grid_state::GridState::{NotWinning, Winning};
use crate::days::day_04::position::GridPosition;
use crate::parse_input;

pub const GRID_SIZE: usize = 5;
const GRID_SIZE_U32: u32 = 5;



#[derive(Clone)]
pub struct Grid {
    numbers: HashMap<u32, GridPosition>,
    columns: [u32; GRID_SIZE],
    rows: [u32; GRID_SIZE],
    state:GridState,
}


impl Grid {
    pub fn parse(lines: [&str; GRID_SIZE]) -> Self {
        let numbers: HashMap<u32, GridPosition> = lines.iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.split_whitespace()
                    .enumerate()
                    .map(move |(column, n)| {
                        (parse_input!(n,u32), GridPosition::of(column , row ))
                    })
            })
            .collect();

        return Grid::new(numbers);
    }

    fn new(numbers: HashMap<u32, GridPosition>) -> Self {
        let columns = [0u32; GRID_SIZE];
        let rows = [0u32; GRID_SIZE];
        Grid { numbers, columns, rows, state:NotWinning }
    }

    pub fn state(&self) -> GridState {
        self.state
    }

    pub fn push_number(&self,number_drawn:u32) -> Self {
        let mut new_grid = self.clone();
        if matches!(self.state,Winning(_)) {
            return new_grid;
        }

        if let Some(position) = new_grid.numbers.remove(&number_drawn) {
            new_grid.columns[position.column()]+=1;
            new_grid.rows[position.row()]+=1;
            if new_grid.columns[position.column()] == GRID_SIZE_U32 || new_grid.rows[position.row()] == GRID_SIZE_U32 {
                let result:u32 = new_grid.numbers.keys().map(|u| *u).sum();
                new_grid.state = Winning(result*number_drawn)
            }
        };
        return new_grid;
    }
}
