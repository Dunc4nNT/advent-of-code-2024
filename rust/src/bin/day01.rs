use anyhow::{Ok, Result};
use itertools::Itertools;

fn get_id_vecs(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple())
        .map(|(left_id, right_id)| (left_id.parse::<u32>().unwrap(), right_id.parse::<u32>().unwrap()))
        .unzip()
}

fn part1(input: &str) -> Result<u32> {
    let (mut left_ids , mut right_ids): (Vec<u32>, Vec<u32>) = get_id_vecs(input);
    left_ids.sort();
    right_ids.sort();

    Ok(
        left_ids
            .iter()
            .zip(right_ids.iter())
            .map(|(i, j)| i.abs_diff(*j))
            .sum()
    )
}

fn part2(input: &str) -> Result<u32> {
    let (left_ids , right_ids): (Vec<u32>, Vec<u32>) = get_id_vecs(input);

    Ok(
        left_ids
            .iter()
            .map(|i| i * right_ids.iter().filter(|&j| j == i).count() as u32)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../../../data/aoc/day01/example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT).unwrap(), 31);
    }
}

fn main() -> Result<()> {
    let input: &str = include_str!("../../../data/aoc/day01/input.txt");

    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);

    Ok(())
}
