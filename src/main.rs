use anyhow::{anyhow, Result};
use aoc::Solution;
use std::env;

mod year2024;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let solution = match args[1].as_str() {
        "2024" => year2024::run(&args[2]),
        _ => Err(anyhow!("invalid year specified")),
    }?;

    println!("Part 1: {}", solution.part1);
    println!("Part 2: {}", solution.part2);

    Ok(())
}
