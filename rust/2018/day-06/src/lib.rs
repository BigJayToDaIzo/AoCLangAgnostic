use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let (minmax, mut coords) = parse_input(inp);
    // mark coords on mins and maxes
    // track coords that meet criteria into a new vec
    // iterate over new vec to mutate hashmap
    for c in coords.values_mut() {
        if c.point.x == minmax.min_x
            || c.point.x == minmax.max_x
            || c.point.y == minmax.min_y
            || c.point.y == minmax.max_y
        {
            c.infinite_plane = true;
        }
    }
    for (i, c) in coords.values().enumerate() {
        if c.infinite_plane {
            dbg!(i, c);
        }
    }
    // calculate manhattan distance on infinite planes
    // but do not seek max value on those points
    dbg!(minmax);
    "".to_string()
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
}

fn parse_input(inp: &str) -> (MinsNMaxs, HashMap<usize, Coord>) {
    let mut coords: HashMap<usize, Coord> = HashMap::new();
    let mut minmax = MinsNMaxs {
        min_x: u32::MAX,
        min_y: u32::MAX,
        max_x: 0,
        max_y: 0,
    };
    for (i, line) in inp.lines().enumerate() {
        let x_y: Vec<u32> = line.split(", ").map(|s| s.parse().unwrap()).collect();
        if x_y[0] < minmax.min_x {
            minmax.min_x = x_y[0];
        }
        if x_y[0] > minmax.max_x {
            minmax.max_x = x_y[0];
        }
        if x_y[1] < minmax.min_y {
            minmax.min_y = x_y[1];
        }
        if x_y[1] > minmax.max_y {
            minmax.max_y = x_y[1];
        }
        coords.insert(
            i,
            Coord {
                point: Point {
                    x: x_y[0],
                    y: x_y[1],
                },
                infinite_plane: false,
            },
        );
    }
    (minmax, coords)
}

// measure all of these between the mins/maxs
#[derive(Debug)]
struct Location {
    point: Point,
    closest_coord: Point,
}

// inputs
#[derive(Debug)]
struct Coord {
    point: Point,
    infinite_plane: bool,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct MinsNMaxs {
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
