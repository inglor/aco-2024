use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let (orders, updates) = read_protocols(input);
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if let Some(values) = orders.get(&update[i]) {
                    if !values.contains(&update[j]) {
                        valid = false;
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            sum += update[update.len() / 2];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let (orders, updates) = read_protocols(input);
    for update in updates {
        let mut valid = true;
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if let Some(values) = orders.get(&update[i]) {
                    if !values.contains(&update[j]) {
                        valid = false;
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            sum += reorder(&orders, &update);
        }
    }
    Some(sum)
}

fn read_protocols(content: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let (input_order, input_updates) = content.split_once("\n\n").unwrap();
    (
        input_order.lines().fold(HashMap::new(), |mut map, order| {
            let (first, second) = order.trim().split_once('|').unwrap();
            map.entry(first.parse().unwrap())
                .or_default()
                .insert(second.parse().unwrap());
            map
        }),
        input_updates
            .lines()
            .map(|update| {
                update
                    .trim()
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

fn reorder(orders: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> u32 {
    let mut sorted = update.to_owned();
    sorted.sort_by(|a, b| {
        if let Some(b_vals) = orders.get(b) {
            if b_vals.contains(a) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    });
    sorted[update.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
