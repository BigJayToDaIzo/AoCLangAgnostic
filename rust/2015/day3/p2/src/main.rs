use std::fs;

fn main() {
    // read in input.txt
    let input = fs::read_to_string("../input.txt").unwrap();
    // dbg!(input);
    // let's create a grid map
    let mut sled = Point { x: 0, y: 0 };
    let mut robo_sled = Point { x: 0, y: 0 };
    let mut grid = vec![Point { x: 0, y: 0 }];
    let mut iter = 0;
    for c in input.chars() {
        // scan char
        match c {
            '^' => {
                if iter % 2 == 0 {
                    sled.y += 1;
                } else {
                    robo_sled.y += 1;
                }
                iter += 1;
            }
            'v' => {
                if iter % 2 == 0 {
                    sled.y -= 1;
                } else {
                    robo_sled.y -= 1;
                }
                iter += 1;
            }
            '<' => {
                if iter % 2 == 0 {
                    sled.x -= 1;
                } else {
                    robo_sled.x -= 1;
                }
                iter += 1;
            }
            '>' => {
                if iter % 2 == 0 {
                    sled.x += 1;
                } else {
                    robo_sled.x += 1;
                }
                iter += 1;
            }
            _ => (),
        }
        // if sled point is in vector do nothing
        if iter % 2 == 0 {
            if !grid.contains(&sled) {
                grid.push(sled);
            }
        } else if !grid.contains(&robo_sled) {
            grid.push(robo_sled);
        }
        // else insert sled point into grid
    }
    dbg!(grid.len());
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
