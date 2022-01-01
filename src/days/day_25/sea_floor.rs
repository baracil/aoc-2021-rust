use std::fmt::{Display, Formatter, Write};
use crate::AOCResult;
use crate::days::day_25::sea_floor::Kind::{South, Empty, East, Sentinel};

#[derive(Clone,Copy)]
pub enum Kind {
    Empty,
    East,
    South,
    Sentinel,
}

impl Kind {

    fn get_char(&self) -> char {
        match self {
            Empty => '.',
            East => '>',
            South => 'v',
            Sentinel => '█',
        }

    }

}


pub struct SeaFloor {
    floors: [Vec<Kind>; 2],
    current_floor: usize,
    floor_size:usize,
    nb_columns: usize,
}


impl Display for SeaFloor {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.floors[self.current_floor]
            .chunks(self.nb_columns)
            .for_each(|row| {
                row.iter().map(Kind::get_char).for_each(|c| f.write_char(c).unwrap());
                f.write_char('\n').unwrap();
            });

        Ok(())
    }
}


impl SeaFloor {

    fn copy_to_buffer(&mut self) {
        let buffer_index = 1-self.current_floor;
        for i in 0..self.floor_size {
            let kind = self.floors[self.current_floor][i];
            self.floors[buffer_index][i] = kind;
        }
    }

    fn is_empty(&self, position:usize) -> bool {
        matches!(self.get_kind(position),Empty)
    }

    fn get_kind(&self, position: usize) -> &Kind {
        &self.floors[self.current_floor][position]
    }

    fn set_kind_to_buffer(&mut self, position: usize, kind: Kind) {
        self.floors[1 - self.current_floor][position] = kind
    }

    fn switch_floors(&mut self) {
        self.current_floor = 1-self.current_floor;
    }


    fn get_next_position_east_of(&self, position: usize) -> usize {
        let result = position + 1;
        let next_type = self.get_kind(result);
        if let Sentinel = next_type {
            result - self.nb_columns + 2
        } else {
            result
        }
    }

    fn get_next_position_south(&self, position: usize) -> usize {
        let result = position + self.nb_columns;
        let next_type = self.get_kind(result);
        if let Sentinel = next_type {
            (result % self.nb_columns) + self.nb_columns
        } else {
            result
        }
    }

    pub fn perform_one_step(&mut self) -> bool {
        let any_east_moved = self.move_east_facing_cucumbers();
        self.switch_floors();
        let any_south_moved = self.move_south_facing_cucumbers();
        self.switch_floors();
        any_east_moved || any_south_moved
    }


    fn move_east_facing_cucumbers(&mut self) -> bool {
        let mut move_occurred = false;

        self.copy_to_buffer();

        for position in self.nb_columns..self.floor_size {
            let kind = self.get_kind(position);
            match kind {
                East => {
                    let next_position = self.get_next_position_east_of(position);
                    if self.is_empty(next_position) {
                        move_occurred = true;
                        self.set_kind_to_buffer(next_position, East);
                        self.set_kind_to_buffer(position, Empty);
                    }
                },
                _ => {}
            }
        };

        move_occurred
    }
    fn move_south_facing_cucumbers(&mut self) -> bool {
        let mut move_occurred = false;

        self.copy_to_buffer();

        for position in 0..self.floor_size {
            let kind = self.get_kind(position);
            match kind {
                South => {
                    let next_position = self.get_next_position_south(position);
                    if self.is_empty(next_position) {
                        move_occurred = true;
                        self.set_kind_to_buffer(next_position, South);
                        self.set_kind_to_buffer(position, Empty);
                    }
                },
                _ => {}
            }
        };

        move_occurred
    }
}


impl SeaFloor {
    pub fn parse(lines: &[String]) -> AOCResult<Self> {
        let nb_rows = lines.len();
        let nb_columns = lines[0].len();
        let seafloor_nb_rows = nb_rows + 2;
        let seafloor_nb_columns = nb_columns + 2;


        let mut floor = vec![Kind::Sentinel; seafloor_nb_columns * seafloor_nb_rows];

        for (row_index, line) in lines.iter().enumerate() {
            for (col_index, char) in line.chars().enumerate() {
                let index = (row_index + 1) * seafloor_nb_columns + (col_index + 1);

                let floor_type = match char {
                    '.' => Empty,
                    '>' => East,
                    'v' => South,
                    _ => return Err(format!("Cannot parse char {}", char))
                };

                floor[index] = floor_type;
            }
        }

        let floor_size = floor.len();

        Ok(SeaFloor { floors: [floor.clone(), floor], current_floor: 0, floor_size, nb_columns: seafloor_nb_columns })
    }
}
