advent_of_code::solution!(3);
use regex::Regex;
use std::sync::LazyLock;

fn regex_part_one() -> &'static Regex {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)+").unwrap());
    &RE
}

fn regex_part_two() -> &'static Regex {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)|do\\(\\)|don't\\(\\)+").unwrap());
    &RE
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = regex_part_one()
        .find_iter(input)
        .map(|cmd| {
            let (a, b) = extract_numbers(cmd.as_str());
            a * b
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ignore: bool = false;
    let mut sum: u32 = 0;
    for captures in regex_part_two().captures_iter(input) {
        let cmd = &captures[0];
        if cmd == "do()" {
            ignore = false
        } else if cmd == "don't()" {
            ignore = true
        } else if !ignore {
            let (a, b) = extract_numbers(cmd);
            sum += a * b
        }
    }
    Some(sum)
}

fn extract_numbers(command: &str) -> (u32, u32) {
    if let Some(start) = command.find('(') {
        if let Some(end) = command.find(')') {
            let (a, b) = &command[start + 1..end]
                .split_once(',')
                .map(|(a, b)| (a.trim(), b.trim()))
                .unwrap();
            return (a.parse().unwrap(), b.parse().unwrap());
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
