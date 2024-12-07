advent_of_code::solution!(7);

fn solves(target: i64, values: &[i64], part2: bool) -> bool {
    if values.len() == 1 {
        return values[0] == target;
    }
    if let Some((last, rest)) = values.split_last() {
        if target % last == 0 && solves(target / last, rest, part2) {
            return true;
        }
        if target >= *last && solves(target - last, rest, part2) {
            return true;
        }
        if part2 {
            let mask = 10i64.pow(last.ilog10() + 1);
            if target % mask == *last && solves(target / mask, rest, part2) {
                return true;
            }
        }
    };
    false
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .filter_map(|lines| lines.split_once(": "))
            .map(|(test, values)| {
                let test = test.parse::<i64>().unwrap();
                let nums = values
                    .split_whitespace()
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect::<Vec<_>>();
                (test, nums)
            })
            .filter(|(test, nums)| solves(*test, nums, false))
            .map(|(a, _)| a)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .filter_map(|lines| lines.split_once(": "))
            .map(|(test, values)| {
                let test = test.parse::<i64>().unwrap();
                let nums = values
                    .split_whitespace()
                    .filter_map(|n| n.parse::<i64>().ok())
                    .collect::<Vec<_>>();
                (test, nums)
            })
            .filter(|(test, nums)| solves(*test, nums, true))
            .map(|(a, _)| a)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
