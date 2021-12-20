use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use itertools::Itertools;
use crate::days::day_08::main::day08_launch;
use crate::days::day_20::rule::Rule;
use crate::days::day_20::state::State;
use crate::days::day_20::state::State::{Dark, Sentinel};

#[derive(Clone)]
pub struct Image {
    border_state: State,
    nb_rows: usize,
    nb_columns: usize,
    data: Vec<State>,
}


impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for idx in 0..self.data.len() {
            if (idx % self.nb_columns) == 0 {
                f.write_char('\n')?
            }
            f.write_char(if self.get_bit_value_at(idx) == 1 {'#'} else {'.'})?
        }
        Ok(())
    }
}

impl Image {

    pub(crate) fn nb_lit_pixels(&self) -> usize {
        (0..self.data.len()).map(|i| self.get_bit_value_at(i)).sum()
    }
}

impl Image {

    pub fn enhance(&self, rule: &Rule) -> Image {
        let mut enhanced = self.clone();
        enhanced.perform_enhancement(self, rule);
        enhanced
    }

    fn perform_enhancement(&mut self, reference: &Image, rule: &Rule) {
        (0..reference.number_of_data())
            .filter(|i| reference.is_not_sentinel(*i))
            .for_each(|i| {
                let index = reference.compute_index(i);
                self.data[i] = rule.new_state(index);
            });


        self.border_state = match &reference.border_state {
            State::Light => rule.new_state(511),
            State::Dark => rule.new_state(0),
            _ => panic!("Bug in my solution")
        }
    }


    fn number_of_data(&self) -> usize {
        self.data.len()
    }
    fn is_not_sentinel(&self, index: usize) -> bool {
        !matches!(self.data[index],  Sentinel)
    }

    fn compute_index(&self, pos: usize) -> usize {
        let above = pos - self.nb_columns;
        let below = pos + self.nb_columns;

        [above - 1, above, above + 1, pos - 1, pos, pos + 1, below - 1, below, below+1]
            .iter()
            .map(|i| self.get_bit_value_at(*i))
            .fold(0, |r, bit| r * 2 + bit)
    }

    fn get_bit_value_at(&self, pos: usize) -> usize {
        match &self.data[pos] {
            State::Light => 1,
            Dark => 0,
            Sentinel => (&self.border_state).try_into().unwrap(), //TODO
        }
    }
}

impl Image {

    pub fn parse(input: &str, border_size: usize) -> Result<Self, String> {
        let lines: Vec<&str> = input.lines().collect();

        let nb_rows = lines.len();
        let nb_cols = lines[0].len();

        let nb_image_rows = nb_rows + border_size * 2 + 2;
        let nb_image_cols = nb_cols + border_size * 2 + 2;

        let mut data = vec![Dark; nb_image_rows * nb_image_cols];


        (0..nb_image_rows).for_each(|r| {
            data[r * nb_image_cols] = Sentinel;
            data[r * nb_image_cols + nb_image_cols - 1] = Sentinel;
        });

        (0..nb_image_cols).for_each(|c| {
            data[c] = Sentinel;
            data[(nb_image_rows - 1) * nb_image_cols + c] = Sentinel;
        });


        for (row_idx, line) in lines.iter().enumerate() {
            let offset = (row_idx + border_size+1) * nb_image_cols + border_size+1;
            for (col_idx, c) in line.chars().enumerate() {
                data[offset + col_idx] = c.try_into()?
            }
        }

        Ok(Image { border_state: Dark, nb_rows: nb_image_rows, nb_columns: nb_image_cols, data })
    }
}