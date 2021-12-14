use std::collections::{VecDeque};

pub struct Map {
    octopuses: Vec<i8>,
    nb_cols: usize,
    nb_rows: usize,
    flashed: Vec<bool>,
    to_flash: VecDeque<usize>,
    nb_flashes: usize,
}

impl Map {
    pub fn nb_flashes(&self) -> usize {
        self.nb_flashes
    }
}

const SENTINEL: i8 = -1;


impl Map {
    pub fn perform_one_step(&mut self) -> bool {
        self.to_flash.clear();
        self.flashed.fill(false);

        self.increase_levels();

        let mut nb_flashes = 0usize;
        while !self.to_flash.is_empty() {
            let i = self.to_flash.pop_front().unwrap();
            if self.perform_flash(i) {
                nb_flashes += 1;
            }
        }
        self.nb_flashes += nb_flashes;

        nb_flashes == (self.nb_cols-2) * (self.nb_rows-2)
    }

    fn perform_flash(&mut self, index: usize) -> bool {
        if self.flashed[index] {
            return false;
        }
        self.flashed[index] = true;
        self.octopuses[index] = 0;

        let nb_cols = self.nb_cols as i32;

        for offsets in &[-1 - nb_cols, -nb_cols, 1 - nb_cols, -1, 1, -1 + nb_cols, nb_cols, 1 + nb_cols] {
            let idx = ((index as i32) + offsets) as usize;
            if self.octopuses[idx] == SENTINEL || self.flashed[idx] {
                continue;
            }
            self.octopuses[idx]+=1;
            if self.octopuses[idx] > 9 {
                self.to_flash.push_back(idx);
            }
        }
        true
    }

    fn increase_levels(&mut self) {
        for i in 0..self.octopuses.len() {
            if self.octopuses[i] == SENTINEL {
                continue;
            }
            self.octopuses[i] += 1;
            if self.octopuses[i] > 9 {
                self.to_flash.push_back(i);
            }
        }
    }
}


impl Map {
    pub fn parse(lines: &[String]) -> Map {
        let nb_rows = lines.len() + 2;
        let nb_cols = lines[0].len() + 2;

        let mut octopuses: Vec<i8> = (0..nb_cols * nb_rows).map(|_| SENTINEL).collect();

        for i in 0..lines.len() {
            lines[i].chars().enumerate()
                .for_each(|(j, c)| {
                    octopuses[1 + (i+1) * nb_cols + j] = c as i8 - '0' as i8
                });
        };

        let flashed = (0..octopuses.len()).map(|_| false).collect();

        Map { octopuses, nb_cols, nb_rows,flashed, nb_flashes: 0, to_flash: VecDeque::new() }
    }
}
