advent_of_code::solution!(4);

const WORD: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),   // right
    (-1, 0),  // left
    (0, 1),   // down
    (0, -1),  // up
    (1, 1),   // right down
    (1, -1),  // right up
    (-1, 1),  // left down
    (-1, -1), // left up
];

struct Matrix {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = read_matrix(input);
    let mut sum: u32 = 0;
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            for direction in DIRECTIONS {
                sum += search(WORD, &matrix, x, y, direction.0, direction.1) as u32;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = read_matrix(input);
    let mut sum: u32 = 0;
    for y in 1..matrix.height - 1 {
        for x in 1..matrix.width - 1 {
            if matrix.data[y][x] != 'A' {
                continue; // discard
            }

            let tl = matrix.data[y - 1][x - 1];
            let tr = matrix.data[y - 1][x + 1];
            let bl = matrix.data[y + 1][x - 1];
            let br = matrix.data[y + 1][x + 1];

            if ((tl == 'M' || tl == 'S') && (br == 'M' || br == 'S') && tl != br)
                && ((tr == 'M' || tr == 'S') && (bl == 'M' || bl == 'S') && tr != bl)
            {
                sum += 1;
            }
        }
    }
    Some(sum)
}

fn search(word: &str, matrix: &Matrix, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    let len = word.len();
    let (width, height) = (matrix.width as isize, matrix.height as isize);

    if (0..len).any(|i| {
        let nx = x as isize + dx * i as isize;
        let ny = y as isize + dy * i as isize;
        nx < 0 || nx >= width || ny < 0 || ny >= height
    }) {
        return false; // out of bounds
    }

    let found: String = (0..len)
        .map(|i| {
            let nx = (x as isize + dx * i as isize) as usize;
            let ny = (y as isize + dy * i as isize) as usize;
            matrix.data[ny][nx]
        })
        .collect();

    found == word
}

fn read_matrix(input: &str) -> Matrix {
    let data: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let height = data.len();
    let width = data[0].len();
    Matrix {
        data,
        height,
        width,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
