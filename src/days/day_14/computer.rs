use std::collections::HashMap;

use crate::days::day_14::distribution::Distribution;
use crate::tools::capital_to_couple_u8;

pub struct DistriComputer<'a> {
    insertion_rules: &'a HashMap<(u8, u8), u8>,
}

impl<'a> DistriComputer<'a> {
    pub fn create(rules: &'a HashMap<(u8, u8), u8>) -> Self {
        DistriComputer { insertion_rules: rules }
    }
}

impl DistriComputer<'_> {
    pub fn compute_for_template(&mut self, template: &str, generation: u32) -> usize {
        let chars: Vec<char> = template.chars().collect();

        let result = chars.windows(2).map(capital_to_couple_u8)
            .map(|(left, right)| {
                let mut cache = HashMap::<(u8,u8,u32),Distribution>::new();
                self.compute_for_couple(left, right, generation, &mut cache).clone()
            })
            .reduce(|d1, d2| (d1 + d2)).unwrap();

        result.amplitude()
    }


    pub fn compute_for_couple(&self, left: u8, right: u8, generation: u32, cache: &mut HashMap<(u8, u8, u32), Distribution>) -> Distribution {
        let cached = cache.get(&(left, right, generation));
        if let Some(dist) = cached {
            return dist.clone();
        }


        let computed = if generation == 0 {
            Distribution::first_generation(left, right)
        } else {
            let middle = self.insertion_rules.get(&(left, right)).copied();

            if let Some(middle) = middle {
                let dist_left = self.compute_for_couple(left, middle, generation - 1, cache);
                let dist_right = self.compute_for_couple(middle, right, generation - 1, cache);
                dist_left + dist_right
            } else {
                Distribution::first_generation(left, right)
            }
        };


        let result = computed.clone();
        cache.insert((left, right, generation), computed);
        result
    }

}