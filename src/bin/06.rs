advent_of_code::solution!(6);

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

type Dirs = Vec<(i32, i32)>;
type MoveResult = ((i32, i32), i32, bool, Vec<(i32, i32)>);

fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start: (i32, i32) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                start = (i as i32, j as i32);
            }
        }
        matrix.push(row);
    }

    (matrix, start)
}

fn move_direction(matrix: &mut [Vec<char>], pos: (i32, i32), dir: (i32, i32)) -> MoveResult {
    let mut move_count = 0;
    let mut new_pos = pos;
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let mut possible_obstacles: Vec<(i32, i32)> = Vec::new();

    loop {
        let old_pos = new_pos;
        new_pos = (old_pos.0 + dir.0, old_pos.1 + dir.1);

        if !(new_pos.0 >= 0 && new_pos.0 < rows && new_pos.1 >= 0 && new_pos.1 < cols) {
            return (
                (new_pos.0, new_pos.1),
                move_count,
                false,
                possible_obstacles,
            );
        } else {
            let value = matrix[new_pos.0 as usize][new_pos.1 as usize];
            if value == '#' {
                return (old_pos, move_count, true, possible_obstacles);
            } else if value != 'X' {
                possible_obstacles.push(new_pos);
                move_count += 1;
                matrix[new_pos.0 as usize][new_pos.1 as usize] = 'X';
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut matrix, pos) = parse_input(input);
    let mut count = 0;

    // Directions UP, RIGHT, DOWN, LEFT
    let dirs: Dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_dir = 0;
    let mut current_pos = pos;
    loop {
        let result = move_direction(&mut matrix, current_pos, dirs[current_dir]);
        current_dir = (current_dir + 1) % dirs.len();
        current_pos = result.0;
        count += result.1;

        if !result.2 {
            break;
        }
    }
    Some((count + 1) as u32)
}

fn find_infinite_loops(matrix: &mut [Vec<char>], pos: (i32, i32), obs: (i32, i32)) -> bool {
    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut clone = matrix.to_owned();
    clone[obs.0 as usize][obs.1 as usize] = '#';

    let mut current_dir = 0;
    let mut current_pos = pos;

    let mut operations: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    loop {
        // Infinite loop found
        if operations.contains(&(current_pos, dirs[current_dir])) {
            return true;
        }
        operations.insert((current_pos, dirs[current_dir]));

        let result = move_direction(&mut clone, current_pos, dirs[current_dir]);
        current_dir = (current_dir + 1) % dirs.len();
        current_pos = result.0;

        if !result.2 {
            return false;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut matrix, pos) = parse_input(input);

    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut possible_blocs: Vec<(i32, i32)> = Vec::new();

    let mut current_dir = 0;
    let mut current_pos = pos;
    loop {
        let result = move_direction(&mut matrix, current_pos, dirs[current_dir]);
        current_dir = (current_dir + 1) % dirs.len();
        possible_blocs.extend(result.3);
        current_pos = result.0;

        if !result.2 {
            break;
        }
    }

    let count = possible_blocs
        .par_iter()
        .filter(|pp| find_infinite_loops(&mut matrix.clone(), pos, **pp))
        .count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
