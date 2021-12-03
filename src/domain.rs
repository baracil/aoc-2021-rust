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


#[derive(Default, Copy, Clone)]
pub struct Submarine {
    position:Position,
    aim:i32,
}



impl Submarine {

    pub fn apply_command_stupid(&self, command:&SubCommand) -> Submarine {
        match command {
            Forward(amount) => self.translate(*amount,0),
            Down(amount) => self.translate(0,-*amount),
            Up(amount) => self.translate(0,*amount),
        }
    }

    pub fn apply_command(&self, command:&SubCommand) -> Submarine {
        match command {
            Down(amount) => self.new_with_aim(self.aim - amount),
            Up(amount) => self.new_with_aim(self.aim + amount),
            Forward(amount) => self.translate(*amount,*amount*self.aim),
        }
    }

    fn new_with_position(&self,position:Position) -> Submarine {
        Submarine{position, aim:self.aim}
    }

    fn new_with_aim(&self,aim:i32) -> Submarine {
        Submarine{position:self.position, aim}
    }

    pub fn horizontal(&self) -> i32 {
        self.position.x
    }

    pub fn depth(&self) -> i32 {
        return -self.position.y;
    }

    fn translate(&self, dx: i32, dy: i32) -> Submarine {
        self.new_with_position(self.position.translate(dx,dy))
    }
}


#[derive(Copy, Clone, Default)]
pub struct Position {
    x:i32,
    y:i32
}

impl Position {

    pub fn translate(&self, dx:i32, dy:i32) -> Position {
        Position{x:self.x+dx,y:self.y+dy}
    }

}
