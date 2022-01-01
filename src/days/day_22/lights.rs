use std::collections::HashSet;
use std::net::UdpSocket;
use itertools::{iproduct, Itertools};
use crate::days::day_22::command::Command;
use crate::days::day_22::range;
use crate::days::day_22::system::System;

#[derive(Default)]
pub struct Lights {
    lights_on: HashSet<(usize, usize, usize)>,
    nb_on:usize,
}


impl Lights {
    pub fn apply_command(&mut self, command: &Command<usize>) {
        iproduct!(
            command.x_range().range(),
            command.y_range().range(),
            command.z_range().range())
            .for_each(|coordinate| {
                if command.light_state {
                    self.lights_on.insert(coordinate);
                } else {
                    self.lights_on.remove(&coordinate);
                };
            });
    }


    pub fn nb_lights_on(&self, system:&System) -> usize {
        self.lights_on
            .iter()
            .map(|coordinate| system.volume_at(coordinate))
            .sum()
    }


}