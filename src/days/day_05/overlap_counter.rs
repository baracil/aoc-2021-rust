use std::collections::HashMap;
use crate::days::day_05::position::Position;

#[derive(Default)]
pub struct OverlapCounter {
    nb_overlaps:u32,
    positions:HashMap<Position,u32>,
}


impl OverlapCounter {

    pub fn nb_overlaps(&self) -> u32 {
        self.nb_overlaps
    }

    pub fn push_position(&mut self, position:&Position) -> &mut Self {
        let nb_vents = self.positions.get(position).cloned().unwrap_or(0);
        if nb_vents == 1 {
            self.nb_overlaps +=1;
        }
        self.positions.insert(*position, nb_vents+1);
        self
    }
}