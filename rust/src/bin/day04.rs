use std::time::{Duration, Instant};

use anyhow::{Ok, Result};

fn part1(input: &str) -> Result<usize> {
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../../data/aoc/day04/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT).unwrap(), 0);
    }
}

fn main() -> Result<()> {
    let input: &str = include_str!("../../../data/aoc/day04/input.txt");

    let part1_start_time: Instant = Instant::now();
    let part1_result: usize = part1(input)?;
    let part1_duration: Duration = part1_start_time.elapsed();
    println!("Day 04 Part 1: {:?} ({:?})", part1_result, part1_duration);

    let part2_start_time: Instant = Instant::now();
    let part2_result: usize = part2(input)?;
    let part2_duration: Duration = part2_start_time.elapsed();
    println!("Day 04 Part 2: {:?} ({:?})", part2_result, part2_duration);

    Ok(())
}
