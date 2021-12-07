use std::str::FromStr;

#[derive(Copy, Clone, Eq, Hash)]
pub struct Position {
    x:i32,
    y:i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Position {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn at(x:i32, y:i32) -> Self{
        Position{x,y}
    }
}


impl FromStr for Position {
    type Err = String;

    fn from_str(point_as_str: &str) -> Result<Self, Self::Err> {
        let tokens:Vec<i32> = point_as_str.split(",")
            .map(str::trim)
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let x = *tokens.get(0).ok_or(format!("Cannot parse Position from {}",point_as_str))?;
        let y = *tokens.get(1).ok_or(format!("Cannot parse Position from {}",point_as_str))?;

        Ok(Position{x,y})
    }
}