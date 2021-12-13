use std::str::FromStr;
use crate::days::day_13::fold::Fold::{X, Y};
use crate::parse_input;

pub enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    fn create_error_msg(instruction: &str) -> String {
        format!("Cannot parse folding instruction '{}'", instruction)
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let info = instruction.split(" ")
            .skip(2)
            .next()
            .ok_or_else(|| Fold::create_error_msg(instruction))?
            .split_once("=")
            .ok_or_else(|| Fold::create_error_msg(instruction))?;

        let pos = parse_input!(info.1,i32);
        match info.0 {
            "x" => Ok(X(pos)),
            "y" => Ok(Y(pos)),
            _ => Err(Fold::create_error_msg(instruction))
        }
    }
}