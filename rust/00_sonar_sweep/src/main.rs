//!
//! 00_sonar_sweep
//! https://adventofcode.com/2021/day/1
//!
use itertools::Itertools;
use std::error::Error;
use std::io::{self, BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let reader = BufReader::new(io::stdin());
    let nums = reader
        .lines()
        .map(|l| Ok(l?.parse::<i64>()?))
        .collect::<Result<Vec<i64>>>()?;

    let part1 = count_increases(&nums);
    println!("Part 1: {}", part1);

    let part2 = count_increases_chunks(&nums);
    println!("Part 2: {}", part2);

    Ok(())
}

fn count_increases(nums: &[i64]) -> i64 {
    return nums
        .into_iter()
        .tuple_windows::<(_, _)>()
        .map(|(lhs, rhs)| if lhs < rhs { 1 } else { 0 })
        .sum();
}

fn count_increases_chunks(nums: &[i64]) -> i64 {
    let vals: Vec<_> = nums
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .map(|v| v.0 + v.1 + v.2)
        .collect();
    count_increases(&vals)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_count_increases() {
        assert_eq!(
            count_increases(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            7
        )
    }

    #[test]
    fn test_count_increases_chunks() {
        assert_eq!(
            count_increases_chunks(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
            5
        )
    }
}
