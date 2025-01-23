advent_of_code::solution!(11);

use advent_of_code::utils::math::DigitCounter;
use cached::proc_macro::cached;
use rayon::prelude::*;

pub fn part_one(input: &str) -> Option<u64> {
    let data: Vec<u32> = input
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    Some(solve(data, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let data: Vec<u32> = input
        .split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    Some(solve(data, 75))
}

fn solve(data: Vec<u32>, blinks: usize) -> u64 {
    data.par_iter().map(|&x| calculate(x as u64, blinks)).sum()
}

fn calculate_step(v: u64) -> Vec<u64> {
    if v == 0 {
        return vec![1];
    }

    let n = v.count_digits() as u32;
    if n % 2 == 0 {
        let split_value = 10_u64.pow(n / 2);
        return vec![v / split_value, v % split_value];
    }

    vec![v * 2024]
}

#[cached]
fn calculate(v: u64, n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    calculate_step(v)
        .into_iter()
        .map(|x| calculate(x, n - 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
