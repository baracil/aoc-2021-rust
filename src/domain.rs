use std::ops::{Deref, Sub};
use std::str::FromStr;
use crate::domain::SubCommand::{Down, Forward, Up};

pub enum SubCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for SubCommand {
    type Err = String;

    fn from_str(command_as_str: &str) -> Result<Self, Self::Err> {
        let tokens:Vec<&str> = command_as_str.trim()
            .splitn(2, " ")
            .collect();

        if tokens.len() != 2 {
            return Err(format!("Cannot parse {}",command_as_str))
        }

        let amount = tokens[1].parse::<i32>().map_err(|o| o.to_string())?;

        match tokens[0] {
            "forward" => Ok(Forward(amount)),
            "down" => Ok(Down(amount)),
            "up" => Ok(Up(amount)),
            _ => Err(format!("Cannot parse {}",command_as_str))
        }
    }
}


pub struct Submarine {
    position:Position,
    aim:i32,
}



impl Submarine {
    pub fn default() -> Self {
        Submarine{position:Position::zero(),aim:0}
    }

    pub fn apply_command_stupid(&mut self, command:&SubCommand) -> &mut Self {
        match command {
            Forward(amount) => self.position.translate(*amount,0),
            Down(amount) => self.position.translate(0,-*amount),
            Up(amount) => self.position.translate(0,*amount),
        };
        self
    }

    pub fn apply_command(&mut self, command:&SubCommand) -> &mut Self {
        match command {
            Down(amount) => self.aim -= *amount,
            Up(amount) => self.aim += *amount,
            Forward(amount) => self.position.translate(*amount,*amount*self.aim),
        };
        self
    }

    pub fn horizontal(&self) -> i32 {
        self.position.x
    }

    pub fn depth(&self) -> i32 {
        return -self.position.y;
    }



}


#[derive(Copy, Clone)]
pub struct Position {
    x:i32,
    y:i32
}

impl Position {
    pub fn zero() -> Self {
        Self{x:0,y:0}
    }

    pub fn translate(&mut self, dx:i32, dy:i32)  {
        self.x += dx;
        self.y += dy;
    }

}
