use std::time::{Duration, Instant};

use anyhow::{Ok, Result};

fn get_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: Vec<u32>, tolerate_amount: u32) -> bool {
    if report.len() <= 2 {
        return true;
    }

    let mut prev: u32 = report[0];
    let small_to_large: bool = report.first() < report.last();
    let mut error_count: u32 = 0;

    for &i in &report[1..] {
        if (prev.abs_diff(i) >= 1) && (prev.abs_diff(i) <= 3) && (prev < i) == small_to_large {
            prev = i;
        } else {
            error_count += 1;
            if error_count > tolerate_amount {
                return false;
            }
        }
    }

    return true;
}

fn part1(input: &str) -> Result<u32> {
    let reports: Vec<Vec<u32>> = get_reports(input);
    let mut safe_report_count: u32 = 0;

    for report in reports {
        if is_report_safe(report.to_vec(), 0) {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

fn part2(input: &str) -> Result<u32> {
    let reports: Vec<Vec<u32>> = get_reports(input);
    let mut safe_report_count: u32 = 0;

    for report in reports {
        if is_report_safe(report.to_vec(), 1) {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../../data/aoc/day02/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT).unwrap(), 4);
    }
}

fn main() -> Result<()> {
    let input: &str = include_str!("../../../data/aoc/day02/input.txt");

    let part1_start_time: Instant = Instant::now();
    let part1_result: u32 = part1(input)?;
    let part1_duration: Duration = part1_start_time.elapsed();
    println!("Day 2 Part 1: {:?} ({:?})", part1_result, part1_duration);

    let part2_start_time: Instant = Instant::now();
    let part2_result: u32 = part2(input)?;
    let part2_duration: Duration = part2_start_time.elapsed();
    println!("Day 2 Part 2: {:?} ({:?})", part2_result, part2_duration);

    Ok(())
}
