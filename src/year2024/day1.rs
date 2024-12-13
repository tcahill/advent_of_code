use anyhow::{Context, Result};
use aoc::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

fn distance(list1: &mut [u64], list2: &mut [u64]) -> u64 {
    list1.sort();
    list2.sort();

    zip(list1, list2).fold(0, |acc, elem| {
        acc + (*elem.1 as i64 - *elem.0 as i64).abs() as u64
    })
}

fn similarity(list1: &[u64], list2: &[u64]) -> u64 {
    let mut counts: HashMap<u64, u64> = HashMap::new();

    for elem in list2 {
        let count = counts.entry(*elem).or_insert(0);
        *count += 1;
    }

    list1.iter().fold(0, |acc, elem| {
        let count = counts.get(elem).unwrap_or(&0);
        acc + (*elem * *count)
    })
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let tuples = input.split_ascii_whitespace()
        .map(|elem| elem.parse().context("parse error"))
        .tuples::<(_, _)>();

    for tuple in tuples {
        list1.push(tuple.0?);
        list2.push(tuple.1?);
    }

    Ok((list1, list2))
}

pub fn run(input: &str) -> Result<Solution> {
    let (mut list1, mut list2) = parse_input(input)?;
    let part1 =  distance(&mut list1, &mut list2);
    let part2 =  similarity(&list1, &list2);
    Ok(Solution{ part1, part2 })
}
