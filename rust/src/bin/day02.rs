use std::time::{Duration, Instant};

use anyhow::{Ok, Result};
use itertools::Itertools;

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

fn is_report_safe(report: Vec<u32>) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let is_small_to_large: bool = report
        .iter()
        .tuple_windows()
        .all(|window: (&u32, &u32)| window.0 < window.1);
    let is_large_to_small: bool = report
        .iter()
        .tuple_windows()
        .all(|window: (&u32, &u32)| window.0 > window.1);

    let has_correct_spacings: bool = report.iter().tuple_windows().all(|window: (&u32, &u32)| {
        (window.0.abs_diff(*window.1) >= 1) && (window.0.abs_diff(*window.1) <= 3)
    });

    return (is_small_to_large || is_large_to_small) && has_correct_spacings;
}

fn is_report_tolerable(report: Vec<u32>) -> bool {
    if is_report_safe(report.to_vec()) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_clone: Vec<u32> = report.clone();
        report_clone.remove(i);

        if is_report_safe(report_clone.to_vec()) {
            return true;
        }
    }

    return false;
}

fn part1(input: &str) -> Result<u32> {
    let reports: Vec<Vec<u32>> = get_reports(input);
    let mut safe_report_count: u32 = 0;

    for report in reports {
        if is_report_safe(report.to_vec()) {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

fn part2(input: &str) -> Result<u32> {
    let reports: Vec<Vec<u32>> = get_reports(input);
    let mut safe_report_count: u32 = 0;

    for report in reports {
        if is_report_tolerable(report.to_vec()) {
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
