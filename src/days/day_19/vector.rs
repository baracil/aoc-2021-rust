use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::try_parse_input;

#[derive(Default,Clone, Eq, PartialEq, Hash,Debug)]
pub struct Vector {
    x:i32,
    y:i32,
    z:i32,
}

impl Vector {
    pub fn manhattan_distance(&self, other: &Vector) -> u32 {
        (self.x - other.x).unsigned_abs()
        +(self.y - other.y).unsigned_abs()
        +(self.z - other.z).unsigned_abs()

    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})",self.x,self.y,self.z))
    }
}

impl Vector {

    pub(crate) fn translate(&self, offset: &Vector) -> Self {
        let x = self.x+offset.x;
        let y = self.y+offset.y;
        let z = self.z+offset.z;
        Vector {x,y,z}
    }

    pub fn rotate(&self, transform:u8) -> Vector {
        match transform/4 {
            0 => {let (y,z) = perform_rotation(transform,self.y, self.z); Vector {x:self.x,y,z}}
            1 => {let (y,z) = perform_rotation(transform,self.y, -self.z); Vector {x:-self.x,y,z}}
            2 => {let (y,z) = perform_rotation(transform,-self.x, self.z); Vector {x:self.y,y,z}}
            3 => {let (y,z) = perform_rotation(transform,self.x, self.z); Vector {x:-self.y,y,z}}
            4 => {let (y,z) = perform_rotation(transform,self.y, -self.x); Vector {x:self.z,y,z}}
            5 => {let (y,z) = perform_rotation(transform,self.y, self.x); Vector {x:-self.z,y,z}}
            _ => panic!("Transform must be lower than 24")
        }
    }


    pub fn subtract(&self, rhs:&Vector) -> Vector {
        Vector {x:self.x - rhs.x, y:self.y - rhs.y, z:self.z - rhs.z}
    }

}

fn perform_rotation(transform:u8, c1:i32, c2:i32) -> (i32,i32) {
    match transform%4 {
        0 => (c1,c2),
        1 => (c2, -c1),
        2 => (-c1,-c2),
        3 => (-c2,c1),
        _ => panic!()
    }
}

impl FromStr for Vector {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let token:Vec<&str> = line.split(',').collect();

        let x = try_parse_input!(token[0],i32)?;
        let y = try_parse_input!(token[1],i32)?;
        let z = try_parse_input!(token[2],i32)?;

        Ok(Vector { x, y, z })
    }
}


