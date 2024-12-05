use regex::Regex;
use std::time::{Duration, Instant};

use anyhow::{Ok, Result};

const MUL_PATTERN: &str = r"mul\((\d{1,3})\,(\d{1,3})\)|(do\(\))|(don\'t\(\))";

fn part1(input: &str, mul_pattern: &Regex) -> Result<u32> {
    let mut total: u32 = 0;

    for mul_match in mul_pattern.captures_iter(input) {
        if mul_match.get(0).unwrap().as_str().starts_with("mul") {
            total += mul_match[1].parse::<u32>().unwrap() * mul_match[2].parse::<u32>().unwrap();
        }
    }

    Ok(total)
}

fn part2(input: &str, mul_pattern: &Regex) -> Result<u32> {
    let mut total: u32 = 0;
    let mut instructions_enabled: bool = true;

    for mul_match in mul_pattern.captures_iter(input) {
        if mul_match.get(0).unwrap().as_str().starts_with("mul") && instructions_enabled {
            total += mul_match[1].parse::<u32>().unwrap() * mul_match[2].parse::<u32>().unwrap();
        } else if mul_match.get(0).unwrap().as_str() == "do()" {
            instructions_enabled = true;
        } else if mul_match.get(0).unwrap().as_str() == "don't()" {
            instructions_enabled = false;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../../data/aoc/day03/example.txt");
    const EXAMPLE_INPUT_PART2: &str = include_str!("../../../data/aoc/day03/example2.txt");

    #[test]
    fn test_part1() {
        let mul_pattern: Regex = Regex::new(MUL_PATTERN).unwrap();

        assert_eq!(part1(EXAMPLE_INPUT, &mul_pattern).unwrap(), 161);
    }

    #[test]
    fn test_part2() {
        let mul_pattern: Regex = Regex::new(MUL_PATTERN).unwrap();

        assert_eq!(part2(EXAMPLE_INPUT_PART2, &mul_pattern).unwrap(), 48);
    }
}

fn main() -> Result<()> {
    let input: &str = include_str!("../../../data/aoc/day03/input.txt");
    let mul_pattern: Regex = Regex::new(MUL_PATTERN).unwrap();

    let part1_start_time: Instant = Instant::now();
    let part1_result: u32 = part1(input, &mul_pattern)?;
    let part1_duration: Duration = part1_start_time.elapsed();
    println!("Day 03 Part 1: {:?} ({:?})", part1_result, part1_duration);

    let part2_start_time: Instant = Instant::now();
    let part2_result: u32 = part2(input, &mul_pattern)?;
    let part2_duration: Duration = part2_start_time.elapsed();
    println!("Day 03 Part 2: {:?} ({:?})", part2_result, part2_duration);

    Ok(())
}
