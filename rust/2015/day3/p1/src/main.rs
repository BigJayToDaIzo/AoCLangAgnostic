use std::fs;

fn main() {
    // read in input.txt
    let input = fs::read_to_string("../input.txt").unwrap();
    // dbg!(input);
    // let's create a grid map
    let mut x = 0;
    let mut y = 0;
    let mut sled = Point { x, y };
    let mut grid = vec![sled];
    for c in input.chars() {
        // scan char
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => (),
        }
        // move sled
        sled = Point { x, y };
        // if sled point is in vector do nothing
        if grid.contains(&sled) {
        } else {
            grid.push(sled);
        }
        // else insert sled point into grid
    }
    dbg!(grid.len());
}

struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
