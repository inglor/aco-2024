use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

type Antenna = (i32, i32);
type AntennasMap = HashMap<char, Vec<Antenna>>;

fn parse_antenas(input: &str) -> (i32, i32, AntennasMap) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            if char != '.' {
                antennas.entry(char).or_default().push((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    (x, y, antennas)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (x, y, antennas) = parse_antenas(input);
    let mut p: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let a_x = positions[i].0 + positions[i].0 - positions[j].0;
                let a_y = positions[i].1 + positions[i].1 - positions[j].1;

                let b_x = positions[j].0 + positions[j].0 - positions[i].0;
                let b_y = positions[j].1 + positions[j].1 - positions[i].1;

                if a_x >= 0 && a_x < x && a_y >= 0 && a_y < y && p.insert((a_x, a_y)) {
                    result += 1;
                }

                if b_x >= 0 && b_x < x && b_y >= 0 && b_y < y && p.insert((b_x, b_y)) {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    let (x, y, antennas) = parse_antenas(input);
    let mut p: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let d_x = positions[i].0 - positions[j].0;
                let d_y = positions[i].1 - positions[j].1;

                let mut a_x = positions[i].0;
                let mut a_y = positions[i].1;

                while a_x >= 0 && a_x < x && a_y >= 0 && a_y < y {
                    if p.insert((a_x, a_y)) {
                        result += 1;
                    }
                    a_x += d_x;
                    a_y += d_y;
                }

                let d_x = positions[j].0 - positions[i].0;
                let d_y = positions[j].1 - positions[i].1;

                let mut b_x = positions[j].0;
                let mut b_y = positions[j].1;

                while b_x >= 0 && b_x < x && b_y >= 0 && b_y < y {
                    if p.insert((b_x, b_y)) {
                        result += 1;
                    }
                    b_x += d_x;
                    b_y += d_y;
                }
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
