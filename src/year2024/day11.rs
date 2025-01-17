use std::collections::HashMap;

use crate::Solution;

use anyhow::{Context, Result};

fn stones_after_n_blinks(stone: u64, iterations: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(answer) = cache.get(&(stone, iterations)) {
        return *answer;
    }

    let result;
    if iterations == 1 {
        if stone == 0 {
            result = 1;
        } else if stone.to_string().len() % 2 == 0 {
            result = 2;
        } else {
            result = 1;
        }
    } else {
        if stone == 0 {
            result = stones_after_n_blinks(1, iterations - 1, cache);
        } else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let stone1 = stone_str[..stone_str.len()/2].parse().unwrap();
            let stone2 = stone_str[stone_str.len()/2..].parse().unwrap();
            result = stones_after_n_blinks(stone1, iterations - 1, cache) + stones_after_n_blinks(stone2, iterations - 1, cache);
        } else {
            result = stones_after_n_blinks(stone * 2024, iterations - 1, cache);
        }
    }

    cache.insert((stone, iterations), result);

    result
}

pub fn run(input: &str) -> Result<Solution> {
    let stones = input.split(' ').map(|x| x.parse().context("parse error")).collect::<Result<Vec<u64>>>()?;
    let part1 = stones.iter().map(|stone| stones_after_n_blinks(*stone, 25, &mut HashMap::new())).reduce(|acc, x| acc + x).unwrap();
    let part2 = stones.iter().map(|stone| stones_after_n_blinks(*stone, 75, &mut HashMap::new())).reduce(|acc, x| acc + x).unwrap();

    Ok(Solution { part1, part2 })
}
