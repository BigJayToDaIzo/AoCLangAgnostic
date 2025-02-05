use std::fs;
use std::process;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let v: Vec<&str> = input.split(", ").collect();
    let mut turn_dir: &str;
    let mut travel_dist: i32;
    let mut santa_pos = Pos {
        pos: Point { x: 0, y: 0 },
        dir: Direction::N,
    };
    let mut visited: Vec<Point> = Vec::new();
    for value in v {
        let value = value.trim();
        turn_dir = &value[0..1];
        travel_dist = value[1..].parse().unwrap();
        santa_pos.dir = new_direction(&santa_pos, turn_dir);
        // fill all blocks in
        match santa_pos.dir {
            Direction::N => {
                travel_and_check_n(&santa_pos.pos, &travel_dist, &mut visited);
                santa_pos.pos.y += travel_dist;
            }
            Direction::S => {
                travel_and_check_s(&santa_pos.pos, &travel_dist, &mut visited);
                santa_pos.pos.y -= travel_dist;
            }
            Direction::E => {
                travel_and_check_e(&santa_pos.pos, &travel_dist, &mut visited);
                santa_pos.pos.x += travel_dist;
            }
            Direction::W => {
                travel_and_check_w(&santa_pos.pos, &travel_dist, &mut visited);
                santa_pos.pos.x -= travel_dist;
            }
        }
    }
}

fn travel_and_check_n(p_start: &Point, travel_dist: &i32, vec: &mut Vec<Point>) {
    let end_block = p_start.y + travel_dist;
    for block in p_start.y + 1..=end_block {
        let p = Point {
            x: p_start.x,
            y: block,
        };
        if !vec.contains(&p) {
            vec.push(p);
        } else {
            dbg!(&vec.len());
            dbg!(p.x.abs() + p.y.abs());
            process::exit(0);
        }
    }
}
//
fn travel_and_check_s(p_start: &Point, travel_dist: &i32, vec: &mut Vec<Point>) {
    let end_block = p_start.y - travel_dist;
    for block in p_start.y - 1..=end_block {
        let p = Point {
            x: p_start.x,
            y: block,
        };
        if !vec.contains(&p) {
            vec.push(p);
        } else {
            dbg!(&vec.len());
            dbg!(p.x.abs() + p.y.abs());
            process::exit(0);
        }
    }
}
fn travel_and_check_e(p_start: &Point, travel_dist: &i32, vec: &mut Vec<Point>) {
    let end_block = p_start.x + travel_dist;
    for block in p_start.x + 1..=end_block {
        let p = Point {
            x: block,
            y: p_start.y,
        };
        if !vec.contains(&p) {
            vec.push(p);
        } else {
            dbg!(&vec.len());
            dbg!(p.x.abs() + p.y.abs());
            process::exit(0);
        }
    }
}
fn travel_and_check_w(p_start: &Point, travel_dist: &i32, vec: &mut Vec<Point>) {
    let end_block = p_start.x - travel_dist;
    for block in p_start.x - 1..=end_block {
        let p = Point {
            x: block,
            y: p_start.y,
        };
        if !vec.contains(&p) {
            vec.push(p);
        } else {
            dbg!(&vec.len());
            dbg!(p.x.abs() + p.y.abs());
            process::exit(0);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Pos {
    pos: Point,
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
