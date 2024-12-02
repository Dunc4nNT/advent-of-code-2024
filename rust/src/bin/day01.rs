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

    const EXAMPLE_INPUT: &str = include_str!("../../../data/aoc/day01/example.txt");

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
    let input: &str = include_str!("../../../data/aoc/day01/input.txt");

    println!("{:?}", part1(input)?);
    println!("{:?}", part2(input)?);

    Ok(())
}
