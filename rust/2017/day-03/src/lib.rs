pub fn read_lines() -> String {
    "289326".to_string()
}

pub fn part_one(inp: &str) -> String {
    let input: u32 = inp.parse().unwrap();
    let g_s_l_parsed: u32;
    let mut grid_side_len: f32 = inp.parse().unwrap();
    if grid_side_len.sqrt().fract() != 0.0 {
        grid_side_len = grid_side_len.sqrt().trunc();
        g_s_l_parsed = grid_side_len as u32 + 1;
    } else {
        g_s_l_parsed = grid_side_len as u32;
    }
    let p = Point { x: 0, y: 0 };
    let mut spiral = vec![Location {
        pos: p.clone(),
        val: 1,
    }];
    // build squares around
    let mut spiral_tracker = SpiralTracker {
        pos: p.clone(),
        heading: Direction::E,
        val: 1,
    };
    for box_side_len in (3..=g_s_l_parsed).step_by(2) {
        let box_range_start = spiral_tracker.val + 1;
        let box_range_end = box_side_len * box_side_len;
        for point_of_square in box_range_start..=box_range_end {
            if spiral_tracker.heading == Direction::E {
                if cannot_move_west(&spiral, &spiral_tracker) { // move up
                     // add point
                } else {
                    spiral_tracker.heading = Direction::E;
                }
            } else if spiral_tracker.heading == Direction::N {
                if cannot_move_south(&spiral, &spiral_tracker) {
                    // add point
                } else {
                    spiral_tracker.hedaing = Direction::S;
                }
            } else if spiral_tracker.heading == Direction::W {
                if cannot_move_east(&spiral, &spiral_tracker) {
                    // add point
                } else {
                    spiral_tracker.heading = Direction::N;
                }
            } else {
            } //run out the range going E
        }
    }
    "".to_string()
}

fn cannot_move_west(spiral: &Vec<Location>, spiral_tracker: &SpiralTracker) -> bool {
    true
}
fn cannot_move_south(spiral: &Vec<Location>, spiral_tracker: &SpiralTracker) -> bool {
    true
}
fn cannot_move_east(spiral: &Vec<Location>, spiral_tracker: &SpiralTracker) -> bool {
    true
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
}

struct SpiralTracker {
    pos: Point,
    heading: Direction,
    val: u32,
}

#[derive(PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    pos: Point,
    val: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = "";

        let res = part_one(&input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = "";

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
