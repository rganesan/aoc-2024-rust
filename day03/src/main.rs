use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::env;
use std::time::Instant;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn sum_results(s: &str) -> Result<u32> {
    let mut sum = 0;
    for cap in RE.captures_iter(s) {
        let a: u32 = cap[1].parse()?;
        let b: u32 = cap[2].parse()?;
        sum += a * b;
    }

    Ok(sum)
}

fn part1(s: &str) -> Result<u32> {
    sum_results(s)
}

fn part2(s: &str) -> Result<u32> {
    let mut sum = 0;
    for s in s.split("do()") {
        let section = s.split_once("don't()").map_or(s, |(before, _)| before);
        sum += sum_results(section)?;
    }
    Ok(sum)
}

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "inputs/input.txt".to_string());

    let s = std::fs::read_to_string(filename)?;

    let start = Instant::now();
    let sum = part1(&s).expect("failed");
    let duration = start.elapsed();
    println!("part1: sum: {sum}, time: {duration:?}");

    let start = Instant::now();
    let sum = part2(&s).expect("failed");
    let duration = start.elapsed();
    println!("part2: sum: {sum}, time: {duration:?}");
    Ok(())
}
