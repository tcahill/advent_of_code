use std::fs;
use anyhow::{anyhow, Result};
use crate::Solution;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

pub fn run(day: &str) -> Result<Solution> {
    match day {
        "1" => day1::run(&fs::read_to_string("input/2024/1")?),
        "2" => day2::run(&fs::read_to_string("input/2024/2")?),
        "3" => day3::run(&fs::read_to_string("input/2024/3")?),
        "4" => day4::run(&fs::read_to_string("input/2024/4")?),
        "5" => day5::run(&fs::read_to_string("input/2024/5")?),
        "6" => day6::run(&fs::read_to_string("input/2024/6")?),
        "7" => day7::run(&fs::read_to_string("input/2024/7")?),
        "8" => day8::run(&fs::read_to_string("input/2024/8")?),
        "9" => day9::run(&fs::read_to_string("input/2024/9")?),
        "10" => day10::run(&fs::read_to_string("input/2024/10")?),
        "11" => day11::run(&fs::read_to_string("input/2024/11")?),
        "12" => day12::run(&fs::read_to_string("input/2024/12")?),
        _ => Err(anyhow!("invalid day specified")),
    }
}
