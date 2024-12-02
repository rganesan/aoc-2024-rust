use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// Find bad level and return the index if found
fn find_bad_level(levels: &[i32]) -> Option<usize> {
    let increasing = levels[0] < levels[1];
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        // println!("{increasing} {i} {diff} {levels:?}");
        if (increasing && diff <= 0) || (!increasing && diff >= 0) || diff.abs() > 3 {
            return Some(i);
        }
    }
    None
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
        if find_bad_level(&levels).is_none() {
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
        let mut levels = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if let Some(i) = find_bad_level(&levels) {
            let mut saved = levels.remove(i);
            if find_bad_level(&levels).is_none() {
                nsafe += 1;
                continue;
            }
            // try removing previous level
            std::mem::swap(&mut levels[i - 1], &mut saved);
            if find_bad_level(&levels).is_none() {
                nsafe += 1;
                continue;
            }
            // try removing the one before, it may reset increase/decrease
            if i > 1 {
                levels[i - 2] = saved;
                if find_bad_level(&levels).is_none() {
                    nsafe += 1;
                    continue;
                }
            }
            // if we got here, removing one level doesn't help, record is unsafe
            continue;
        }
        nsafe += 1;
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
