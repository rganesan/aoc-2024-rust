use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn part1(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let loc_ids = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        left.push(loc_ids[0]);
        right.push(loc_ids[1]);
    }
    left.sort();
    right.sort();

    let mut distance = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        distance += (l - r).abs();
    }

    Ok(distance)
}

fn part2(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut left = Vec::new();
    let mut right = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let loc_ids = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        left.push(loc_ids[0]);
        *right.entry(loc_ids[1]).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for l in left.iter() {
        let repeats = right.get(l).unwrap_or(&0);
        similarity += l * repeats;
    }

    Ok(similarity)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());

    let start = Instant::now();
    let distance = part1(&filename).expect("failed");
    let duration = start.elapsed();
    println!("part1: distance: {distance}, time: {duration:?}");

    let start = Instant::now();
    let similarity = part2(&filename).expect("failed");
    let duration = start.elapsed();
    println!("part2: similarity: {similarity}, time: {duration:?}");
    Ok(())
}
