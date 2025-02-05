use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let v: Vec<&str> = input.split(", ").collect();
    let mut turn_dir: &str;
    let mut travel_dist: i32;
    let mut santa_pos = Pos {
        x: 0,
        y: 0,
        dir: Direction::N,
    };
    for value in v {
        let value = value.trim();
        turn_dir = &value[0..1];
        travel_dist = value[1..].parse().unwrap();
        // calculate new direction
        santa_pos.dir = new_direction(&santa_pos, turn_dir);
        // move santa tavel_dist in that direction
        match santa_pos.dir {
            Direction::N => {
                santa_pos.y += travel_dist;
            }
            Direction::S => {
                santa_pos.y -= travel_dist;
            }
            Direction::E => {
                santa_pos.x += travel_dist;
            }
            Direction::W => {
                santa_pos.x -= travel_dist;
            }
        }
    }
    dbg!(santa_pos.x.abs() + santa_pos.y.abs());
    // figure out how far santa is from origin (abs|x| + abs|y|)
}

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
    dir: Direction,
}

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn new_direction(p: &Pos, d: &str) -> Direction {
    let current_dir = &p.dir;
    match current_dir {
        Direction::N => {
            if d == "L" {
                Direction::W
            } else {
                Direction::E
            }
        }
        Direction::S => {
            if d == "L" {
                Direction::E
            } else {
                Direction::W
            }
        }
        Direction::E => {
            if d == "L" {
                Direction::N
            } else {
                Direction::S
            }
        }
        Direction::W => {
            if d == "L" {
                Direction::S
            } else {
                Direction::N
            }
        }
    }
}
