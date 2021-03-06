use std::cmp::Ordering;
use std::str::FromStr;
use crate::days::day_05::position::Position;

pub struct Vent {
    start: Position,
    end: Position,
}

impl Vent {

    pub fn is_vertical_or_horizontal(&self) -> bool {
        self.start.x() == self.end.x() || self.start.y() == self.end.y()
    }

    pub fn lines(&self) -> impl Iterator<Item=Position> {
        let x_range = self.end.x() - self.start.x();
        let y_range = self.end.y() - self.start.y();

        let delta = x_range.abs().max(y_range.abs());

        let dx = sign(x_range);
        let dy = sign(y_range);

        let start_x = self.start.x();
        let start_y = self.start.y();
        (0_i32..=delta).map(move |i| Position::at(start_x + (i as i32) * dx, start_y + (i as i32) * dy))
    }
}

pub fn sign(value: i32) -> i32 {
    match value.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1
    }
}

impl FromStr for Vent {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Position> = line.split("->")
            .map(str::trim)
            .filter_map(|pos| pos.parse::<Position>().ok())
            .collect();

        let start = positions.get(0).cloned().ok_or(format!("Cannot parse Vent from {}", line))?;
        let end = positions.get(1).cloned().ok_or(format!("Cannot parse Vent from {}", line))?;

        Ok(Vent { start, end })
    }
}

