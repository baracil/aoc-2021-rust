use crate::{AOCResult, day18_launch, day19_launch, Part};
use crate::days::day_01::main::day01_launch;
use crate::days::day_02::main::day02_launch;
use crate::days::day_03::main::day03_launch;
use crate::days::day_04::main::day04_launch;
use crate::days::day_05::main::day05_launch;
use crate::days::day_06::main::day06_launch;
use crate::days::day_07::main::day07_launch;
use crate::days::day_08::main::day08_launch;
use crate::days::day_09::main::day09_launch;
use crate::days::day_10::main::day10_launch;
use crate::days::day_11::main::day11_launch;
use crate::days::day_12::main::day12_launch;
use crate::days::day_13::main::day13_launch;
use crate::days::day_14::main::day14_launch;
use crate::days::day_15::main::day15_launch;
use crate::days::day_16::main::day16_launch;
use crate::days::day_17::main::day17_launch;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Day {
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
}

impl Day {
    pub fn solver(&self) -> impl Fn(Part) -> AOCResult<String> {
        match self {
            Day::D01 => day01_launch,
            Day::D02 => day02_launch,
            Day::D03 => day03_launch,
            Day::D04 => day04_launch,
            Day::D05 => day05_launch,
            Day::D06 => day06_launch,
            Day::D07 => day07_launch,
            Day::D08 => day08_launch,
            Day::D09 => day09_launch,
            Day::D10 => day10_launch,
            Day::D11 => day11_launch,
            Day::D12 => day12_launch,
            Day::D13 => day13_launch,
            Day::D14 => day14_launch,
            Day::D15 => day15_launch,
            Day::D16 => day16_launch,
            Day::D17 => day17_launch,
            Day::D18 => day18_launch,
            Day::D19 => day19_launch,
        }

    }
}