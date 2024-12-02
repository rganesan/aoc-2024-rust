use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn is_safe(levels: &[i32]) -> bool {
    let increasing = levels[0] < levels[1];
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        // println!("{increasing} {i} {diff} {levels:?}");
        if (increasing && diff <= 0) || (!increasing && diff >= 0) || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn part1(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut nsafe = 0;
    for line in reader.lines() {
        let line = line?;
        let levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&levels) {
            nsafe += 1;
        }
    }

    Ok(nsafe)
}

fn part2(filename: &str) -> Result<u32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut nsafe = 0;
    for line in reader.lines() {
        let line = line?;
        let levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if is_safe(&levels) {
            nsafe += 1;
            continue;
        }
        for i in 0..levels.len() {
            let mut levels = levels.clone();
            levels.remove(i);
            if is_safe(&levels) {
                nsafe += 1;
                break;
            }
        }
    }

    Ok(nsafe)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());

    let start = Instant::now();
    let nsafe = part1(&filename).expect("failed");
    let duration = start.elapsed();
    println!("part1: nsafe: {nsafe}, time: {duration:?}");

    let start = Instant::now();
    let nsafe = part2(&filename).expect("failed");
    let duration = start.elapsed();
    println!("part2: nsafe: {nsafe}, time: {duration:?}");
    Ok(())
}
