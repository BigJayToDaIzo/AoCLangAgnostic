pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut grid: Vec<Vec<char>> = vec![vec![]; inp.lines().count()];
    for (i, line) in inp.lines().enumerate() {
        grid[i] = line.chars().collect();
    }
    let mut running_total: i32 = 0;
    // scan for digit begin and end
    let mut part_num_string: Option<String> = None;
    for (y, _) in grid.iter().enumerate() {
        for (x, _) in grid[y].iter().enumerate() {
            if grid[y][x].is_numeric() {
                // do stuff
                match part_num_string {
                    Some(mut str) => {
                        str.push(grid[y][x]);
                    }
                    None => {
                        part_num_string = Some(String::from(grid[y][x]));
                    }
                }
            }
        }
    }
    // scan around number for symbols
    // if symbol found, add part num to running total
    running_total.to_string()
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
}

fn check_valid_part(grid: &[Vec<char>], mut x: usize, y: usize) -> (i32, usize) {
    let mut part_num_string = String::from(grid[y][x]);
    // how do we isolate part numbers?
    while x + 1 < grid.len() - 1 && grid[y][x + 1].is_numeric() {
        x += 1;
        part_num_string.push(grid[y][x]);
    }
    // how do we scan AROUND them for symbols after?
    // let's review the string methods.
    (part_num_string.parse().unwrap(), x)
}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "abc");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "abc");
    }
}
