use std::collections::HashMap;
use crate::days::day_23::burrow::Burrow;

pub struct Organizer {
    cache:HashMap<String,Option<usize>>,
}

impl Organizer {

    pub fn organize(burrow:&Burrow) -> Option<usize> {
        let mut organizer = Organizer{cache:HashMap::new()};
        organizer.minimal_energy_to_organize(burrow)
    }

    fn minimal_energy_to_organize(&mut self, burrow:&Burrow) -> Option<usize> {
        if burrow.is_organized() {
            return Some(0);
        }

        let cached = self.cache.get(burrow.map());

        if let Some(cached) = cached {
            return cached.to_owned()
        }


        let displacements = burrow.allowed_displacements();

        let lowest_energy = displacements
            .iter()
            .map(|d| (burrow.with_moved_amphipod( d),d.energy_cost()))
            .map(|b| self.minimal_energy_to_organize(&b.0).map(|e| e+b.1))
            .flatten()
            .min();

        self.cache.insert(burrow.map().clone(),lowest_energy);

        lowest_energy
    }


}