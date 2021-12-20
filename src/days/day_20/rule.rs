use std::str::FromStr;
use crate::days::day_20::state::State;
use crate::days::day_20::state::State::{Dark, Light};

pub struct Rule {
    rules:Vec<State>,
}

impl Rule {
    pub fn new_state(&self, index:usize) -> State {
        self.rules[index].clone()
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(rule_as_string: &str) -> Result<Self, Self::Err> {
        let rules = rule_as_string.chars().map(|c| if c == '#' { Light } else { Dark }).collect();
        Ok(Rule { rules })
    }
}