const MIN_X: i32 = 0;
const MAX_X: i32 = 129;
const MIN_Y: i32 = 0;
const MAX_Y: i32 = 129;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut guard_location = GuardPos {
        pos: Point { x: 0, y: 0 },
        heading: Direction::North,
    };
    let mut visited: Vec<Point> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    for (y, line) in inp.lines().enumerate() {
        if let Some(x) = line.chars().position(|c| c == '^') {
            guard_location.pos.x = x as i32;
            guard_location.pos.y = y as i32;
            visited.push(guard_location.pos);
        }
        grid.push(line.chars().collect());
    }
    loop {
        if moves_off_grid(guard_location) {
            break;
        } else {
            move_guard(&mut guard_location, &grid, &mut visited);
        }
    }
    visited.len().to_string()
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
}

fn moves_off_grid(gp: GuardPos) -> bool {
    match gp.heading {
        Direction::North => {
            if gp.pos.y == MIN_Y {
                return true;
            }
        }
        Direction::South => {
            if gp.pos.y == MAX_Y {
                return true;
            }
        }
        Direction::East => {
            if gp.pos.x == MAX_X {
                return true;
            }
        }
        Direction::West => {
            if gp.pos.x == MIN_X {
                return true;
            }
        }
    }
    false
}

fn move_guard(gp: &mut GuardPos, grid: &[Vec<char>], visited: &mut Vec<Point>) {
    // do not need to check grid bounds because moves_off_grid() already checks
    match gp.heading {
        Direction::North => {
            if grid[(gp.pos.y - 1) as usize][gp.pos.x as usize] == '#' {
                gp.heading = Direction::East;
            } else {
                gp.pos.y -= 1;
                check_visited(gp.pos, visited);
            }
        }
        Direction::South => {
            if grid[(gp.pos.y + 1) as usize][gp.pos.x as usize] == '#' {
                gp.heading = Direction::West;
            } else {
                gp.pos.y += 1;
                check_visited(gp.pos, visited);
            }
        }
        Direction::East => {
            if grid[gp.pos.y as usize][(gp.pos.x + 1) as usize] == '#' {
                gp.heading = Direction::South;
            } else {
                gp.pos.x += 1;
                check_visited(gp.pos, visited);
            }
        }
        Direction::West => {
            if grid[gp.pos.y as usize][(gp.pos.x - 1) as usize] == '#' {
                gp.heading = Direction::North;
            } else {
                gp.pos.x -= 1;
                check_visited(gp.pos, visited);
            }
        }
    }
}

fn check_visited(pt: Point, visited: &mut Vec<Point>) {
    if !visited.contains(&pt) {
        visited.push(pt);
    }
}

#[derive(Debug, Copy, Clone)]
struct GuardPos {
    pos: Point,
    heading: Direction,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "5129");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "abc");
    }
}
