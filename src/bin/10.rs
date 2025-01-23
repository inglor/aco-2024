advent_of_code::solution!(10);

use advent_of_code::utils::grid::GridPointsIterator;
use advent_of_code::utils::hash::FastMapBuilder;
use advent_of_code::utils::point::{Point, ORTHOGONAL};
use advent_of_code::utils::{grid::Grid, hash::FastMap};

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);
    let result = part_x(grid).len() as u32;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);
    let result = part_x(grid).values().sum();
    Some(result)
}

fn part_x(grid: Grid<u8>) -> FastMap<(Point, Point), u32> {
    let mut result = FastMap::new();
    let mut paths = vec![];
    for start_position in grid.points().filter(|&p| grid[p] == b'0') {
        paths.push((b'0', start_position));

        while let Some((height, location)) = paths.pop() {
            if height == b'9' {
                *result.entry((start_position, location)).or_insert(0) += 1;
                continue;
            }
            for next_location in ORTHOGONAL.map(|o| location + o) {
                if grid.contains(next_location) && grid[next_location] == height + 1 {
                    paths.push((height + 1, next_location));
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
