use std::fmt::{Display, Formatter};
use crate::days::day_23::amphipod::Amphipod;
use crate::days::day_23::consts::HALLWAY_Y;
use crate::days::day_23::position::Position;

#[derive(Debug, Clone)]
pub struct Displacement {
    amphipod_id:usize,
    start:Position,
    end:Position,
    energy_cost:usize,
}

impl Displacement {
    pub fn amphipod_id(&self) -> usize {
        self.amphipod_id
    }
    pub fn start(&self) -> &Position {
        &self.start
    }
    pub fn end(&self) -> &Position {
        &self.end
    }
    pub fn energy_cost(&self) -> usize {
        self.energy_cost
    }

    pub  fn positions<'a>(&'a self) -> impl Iterator<Item=Position> + 'a {
        let up = (HALLWAY_Y+1..=self.start.y).rev().map(|y| Position::of(self.start.x,y));
        let down = (HALLWAY_Y+1..=self.end.y).map(|y| Position::of(self.end.x,y));

        let hallway:Box<dyn Iterator<Item=i32>> = {
            if self.start.x < self.end.x {
                Box::new(self.start.x..=self.end.x)
            } else {
                Box::new((self.end.x..=self.start.x).rev())
            }
        };

        up.chain(hallway.map(|x| Position::of(x,HALLWAY_Y))).chain(down)
    }
}

impl Display for Displacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}",self.start,self.end))
    }
}

impl Displacement {
    pub fn create(amphipod:&Amphipod, end:Position) -> Displacement {
        let start = amphipod.position().clone();
        let energy_cost = start.distance(&end) * amphipod.required_energy();

        Displacement{ amphipod_id:amphipod.id(),start,end,energy_cost }
    }

    pub fn displace(&self, amphipod:&Amphipod) -> Amphipod {
        if self.amphipod_id == amphipod.id() {
            amphipod.with_position(self.end.clone())
        } else {
            amphipod.clone()
        }
    }
}