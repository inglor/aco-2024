advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let report = parse_input(input);
    let sum = report
        .into_iter()
        .filter(|levels| check_levels(levels))
        .count();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let report = parse_input(input);

    let sum = report
        .into_iter()
        .filter(|levels| {
            (0..levels.len()).any(|skip| {
                let mut levels = levels.clone();
                levels.remove(skip);
                check_levels(&levels)
            })
        })
        .count();

    Some(sum)
}

fn check_levels(levels: &[i32]) -> bool {
    let direction = levels[1] > levels[0];
    levels.windows(2).all(|w| {
        let dist: i32 = w[0] - w[1];
        (w[1] > w[0]) == direction && (-3..=3).contains(&dist) && dist != 0
    })
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines();
    let mut reports = vec![];

    for line in lines {
        let level: Vec<i32> = line
            .split_whitespace()
            .filter_map(|number_str| number_str.parse::<i32>().ok())
            .collect();
        reports.push(level);
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
