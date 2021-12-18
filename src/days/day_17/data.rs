use std::str::FromStr;
use crate::parse_input;

#[derive(Debug)]
pub struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}


pub struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Target {
    pub fn xmax(&self) -> i32 {
        self.xmax
    }
    pub fn ymin(&self) -> i32 {
        self.ymin
    }
    pub fn ymax(&self) -> i32 {
        self.ymax
    }
}


impl Probe {
    pub fn create(vx: i32, vy: i32) -> Self {
        Probe { x: 0, y: 0, vx, vy }
    }

    pub fn apply_one_tick(&self) -> Self{
        let x = self.x + self.vx;
        let y = self.y + self.vy;
        let vx = self.vx - self.vx.signum();
        let vy = self.vy - 1;

        Probe{x,y,vx,vy}
    }

    pub fn is_inside(&self, target: &Target) -> bool {
        target.xmax >= self.x
            && target.xmin <= self.x
            && target.ymax >= self.y
            && target.ymin <= self.y
    }

    pub fn missed_target(&self, target: &Target) -> bool {
        target.ymin > self.y || target.xmax < self.x
    }

}


impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Vec<i32> = s.split(|c: char| !c.is_digit(10) && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| parse_input!(s,i32))
            .collect();

        let ymin = result[2].min(result[3]);
        let ymax = result[2].max(result[3]);

        let xmin = result[0].min(result[1]);
        let xmax = result[0].max(result[1]);

        Ok(Target { xmin, xmax, ymin, ymax })
    }
}

impl Target {
}


