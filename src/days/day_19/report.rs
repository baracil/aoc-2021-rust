use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

use crate::days::day_19::vector::Vector;
use crate::parse_input;

#[derive(Debug)]
pub struct Report {
    beacons: HashSet<Vector>,
}

impl Report {
    pub(crate) fn nb_beacons(&self) -> usize {
        self.beacons.len()
    }
}

impl Report {
    pub fn _beacons(&self) -> &HashSet<Vector> {
        &self.beacons
    }

    pub fn merge(&mut self, other:Report) {
        self.beacons.extend(other.beacons);
    }

}

#[derive(Debug)]
pub struct Match {
    report: Report,
    offset:Vector,
    nb_common: usize,
}

impl Match {

    pub fn offset(self) -> Vector {
        self.offset
    }

    pub fn report(self) -> Report {
        self.report
    }
}


impl FromStr for Report {
    type Err = String;

    fn from_str(report: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = report.split('\n').collect();

        let beacons = lines.iter().skip(1).map(|l| parse_input!(l,Vector)).collect();
        Ok(Report { beacons })
    }
}

impl Report {

    pub fn translate(&self, offset: &Vector) -> Report {
        let beacons = self.beacons.iter().map(|p| p.translate(offset)).collect();

        Report { beacons }
    }

    pub fn rotate(&self, rotation: u8) -> Report {
        let beacons = self.beacons.iter().map(|p| p.rotate(rotation)).collect();

        Report { beacons }
    }

    pub fn find_match(&self, report: &Report) -> Option<Match> {
        (0..24)
            .map(|rotation_idx| self.find_offset(rotation_idx, report))
            .flatten()
            .max_by(|t1, t2| t1.nb_common.cmp(&t2.nb_common))
    }

    pub fn find_offset(&self, rotation_idx: u8, report: &Report) -> Option<Match> {
        let transformed_report = report.rotate(rotation_idx);

        let offsets = self.beacons.iter()
            .flat_map(|beacon1| transformed_report.beacons.iter().map(|beacon2| beacon1.subtract(beacon2)))
            .counts_by(|o| o);

        offsets
            .iter()
            .max_by(|o1, o2| o1.1.cmp(o2.1))
            .map(|(p, n)| Match { report:transformed_report.translate(p), nb_common: *n, offset:p.clone() })
            .filter(|m| m.nb_common >= 12)
    }
}

