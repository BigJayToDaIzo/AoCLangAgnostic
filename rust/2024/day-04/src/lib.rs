const PUZZLE_LEN: usize = 140;
pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    // create puzzle
    let puzzle = build_puzzle(inp);
    // count XMAS' in puzzle
    let mut xmas_count = 0;
    for col in 0..PUZZLE_LEN {
        for row in 0..PUZZLE_LEN {
            if puzzle[col][row] == 'X' {
                xmas_count += check_eight(col, row, &puzzle);
            }
        }
    }
    xmas_count.to_string()
}

pub fn part_two(inp: &str) -> String {
    let puzzle = build_puzzle(inp);
    let mut xmas_count = 0;
    for col in 1..PUZZLE_LEN - 1 {
        for row in 1..PUZZLE_LEN - 1 {
            if puzzle[col][row] == 'A' {
                xmas_count += check_cross(col, row, &puzzle);
            }
        }
    }
    xmas_count.to_string()
}

fn check_cross(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if (p[c + 1][r + 1] == 'M' && p[c - 1][r - 1] == 'S'
        || p[c + 1][r + 1] == 'S' && p[c - 1][r - 1] == 'M')
        && (p[c - 1][r + 1] == 'M' && p[c + 1][r - 1] == 'S'
            || p[c - 1][r + 1] == 'S' && p[c + 1][r - 1] == 'M')
    {
        return 1;
    }
    0
}

fn build_puzzle(inp: &str) -> Vec<Vec<char>> {
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in inp.lines() {
        let row: Vec<char> = line.chars().collect();
        puzzle.push(row);
    }
    puzzle
}

fn check_eight(col: usize, row: usize, puzzle: &[Vec<char>]) -> i32 {
    let mut xmas_count = 0;
    // can we check right?
    if col + 3 < PUZZLE_LEN {
        xmas_count += scan_right(col, row, puzzle);
    }
    // can we check down/right?
    if col + 3 < PUZZLE_LEN && row + 3 < PUZZLE_LEN {
        xmas_count += scan_down_right(col, row, puzzle);
    }
    // can we check down?
    if row + 3 < PUZZLE_LEN {
        xmas_count += scan_down(col, row, puzzle);
    }
    // can we check down/left?
    if col as i32 - 3 >= 0 && row + 3 < PUZZLE_LEN {
        xmas_count += scan_down_left(col, row, puzzle);
    }
    // can we check left?
    if col as i32 - 3 >= 0 {
        xmas_count += scan_left(col, row, puzzle);
    }
    // can we check up/left?
    if col as i32 - 3 >= 0 && row as i32 - 3 >= 0 {
        xmas_count += scan_up_left(col, row, puzzle);
    }
    // can we check up?
    if row as i32 - 3 >= 0 {
        xmas_count += scan_up(col, row, puzzle);
    }
    // can we check up/right?
    if col + 3 < PUZZLE_LEN && row as i32 - 3 >= 0 {
        xmas_count += scan_up_right(col, row, puzzle);
    }
    xmas_count
}

fn scan_right(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c + 1][r] == 'M' && p[c + 2][r] == 'A' && p[c + 3][r] == 'S' {
        return 1;
    }
    0
}

fn scan_down_right(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c + 1][r + 1] == 'M' && p[c + 2][r + 2] == 'A' && p[c + 3][r + 3] == 'S' {
        return 1;
    }
    0
}

fn scan_down(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c][r + 1] == 'M' && p[c][r + 2] == 'A' && p[c][r + 3] == 'S' {
        return 1;
    }
    0
}

fn scan_down_left(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c - 1][r + 1] == 'M' && p[c - 2][r + 2] == 'A' && p[c - 3][r + 3] == 'S' {
        return 1;
    }
    0
}

fn scan_left(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c - 1][r] == 'M' && p[c - 2][r] == 'A' && p[c - 3][r] == 'S' {
        return 1;
    }
    0
}

fn scan_up_left(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c - 1][r - 1] == 'M' && p[c - 2][r - 2] == 'A' && p[c - 3][r - 3] == 'S' {
        return 1;
    }
    0
}

fn scan_up(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c][r - 1] == 'M' && p[c][r - 2] == 'A' && p[c][r - 3] == 'S' {
        return 1;
    }
    0
}

fn scan_up_right(c: usize, r: usize, p: &[Vec<char>]) -> i32 {
    if p[c + 1][r - 1] == 'M' && p[c + 2][r - 2] == 'A' && p[c + 3][r - 3] == 'S' {
        return 1;
    }
    0
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "2504");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "1923");
    }
}
