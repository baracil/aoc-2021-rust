use crate::days::day_20::state::State::{Dark, Light};

#[derive(Clone,Debug)]
pub enum State {
    Light,
    Dark,
    Sentinel,
}

impl TryFrom<State> for usize {
    type Error = String;

    fn try_from(value: State) -> Result<Self, Self::Error> {
        match value {
            Light => Ok(1),
            Dark => Ok(0),
            State::Sentinel => Err(format!("Cannot into state '{:?}' to usize",value))
        }
    }
}

impl TryFrom<&State> for usize {
    type Error = String;

    fn try_from(value: &State) -> Result<Self, Self::Error> {
        match value {
            Light => Ok(1),
            Dark => Ok(0),
            State::Sentinel => Err(format!("Cannot into state '{:?}' to usize",value))
        }
    }
}

impl TryFrom<char> for State {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Dark),
            '#' => Ok(Light
            ),
            _ => Err(format!("Cannot parse '{}' to a State",value))
        }
    }
}

