use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use crate::days::day_13::dot::Dot;
use crate::days::day_13::fold::Fold;
use crate::parse_input;

#[derive(Clone)]
pub struct Sheet {
    dots:HashSet<Dot>,
}

impl Display for Sheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xmax = self.dots.iter().map(|d| d.x()).max().unwrap_or(0);
        let ymax = self.dots.iter().map(|d| d.y()).max().unwrap_or(0);

        for y in 0..=ymax {
            f.write_char('\n')?;
            for x in 0..=xmax {
                let dot = Dot::with(x,y);
                if self.dots.contains(&dot) {
                    f.write_char('█')?;
                } else {
                    f.write_char(' ')?;
                }
            }
        }
        Ok(())
    }
}

impl Sheet {

    pub fn fold(&self, fold_instruction:&Fold) -> Self {
        let dots = self.dots.iter().map(|d| d.fold(fold_instruction)).collect::<HashSet<_>>();
        Sheet{dots}
    }


    pub fn parse(dots:&str) -> Sheet {
        let dots = dots.split('\n').map(|l| parse_input!(l,Dot)).collect::<HashSet<_>>();
        Sheet{dots}
    }

    pub fn number_of_dots(&self) -> usize {
        self.dots.len()
    }
}