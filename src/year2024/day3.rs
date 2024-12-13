use anyhow::Result;
use regex::Regex;
use crate::Solution;

fn sum_of_muls(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input).fold(0, |acc, capture| {
        acc + capture[1].parse::<u64>().unwrap_or(0) * capture[2].parse::<u64>().unwrap_or(0)
    })
}

fn sum_of_muls_with_conditionals(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for capture in re.captures_iter(input) {
        match &capture[0][..3] {
            "do(" => { enabled = true; },
            "don" => { enabled = false; },
            "mul" => {
                if enabled {
                    sum += capture[1].parse::<u64>().unwrap_or(0) * capture[2].parse::<u64>().unwrap_or(0);
                }
            },
            _ => {},
        }
    }

    sum
}

pub fn run(input: &str) -> Result<Solution> {
    let part1 = sum_of_muls(input);
    let part2 = sum_of_muls_with_conditionals(input);
    Ok(Solution { part1, part2 })
}
