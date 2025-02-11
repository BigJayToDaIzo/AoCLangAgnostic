pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut grid = [[0u32; 1000]; 1000];
    for claim in parse_claims(inp) {
        grid = claim_into_grid(&claim, grid);
    }
    count_grid_over_twos(grid)
}

pub fn part_two(inp: &str) -> String {
    let mut grid = [[0u32; 1000]; 1000];
    let claims = parse_claims(inp);
    for claim in &claims {
        grid = claim_into_grid(&claim, grid);
    }
    for claim in &claims {
        let mut all_ones = true;
        for x in 0..claim.rect.width {
            let rect_x = (x + claim.rect.in_from_left) as usize;
            for y in 0..claim.rect.height {
                let rect_y = (y + claim.rect.in_from_top) as usize;
                if grid[rect_x][rect_y] != 1 {
                    all_ones = false;
                    break;
                }
            }
        }
        if all_ones {
            return claim.id.to_string();
        }
    }
    "".to_string()
}

fn claim_into_grid(c: &Claim, mut g: [[u32; 1000]; 1000]) -> [[u32; 1000]; 1000] {
    for x in 0..c.rect.width {
        let rect_x = (x + c.rect.in_from_left) as usize;
        for y in 0..c.rect.height {
            let rect_y = (y + c.rect.in_from_top) as usize;
            g[rect_x][rect_y] += 1;
        }
    }
    g
}

fn count_grid_over_twos(g: [[u32; 1000]; 1000]) -> String {
    let mut over_two_counter = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if g[x][y] > 1 {
                over_two_counter += 1;
            }
        }
    }
    over_two_counter.to_string()
}

fn parse_claims(inp: &str) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    for line in inp.lines() {
        let live_vec: Vec<&str> = line.split_whitespace().collect();
        let id: u32 = live_vec[0].replace("#", "").parse().unwrap();
        let mut offsets: Vec<&str> = live_vec[2].split(",").collect();
        offsets[1] = offsets[1].trim_matches(':');
        let side_lens: Vec<&str> = live_vec[3].split("x").collect();
        let in_from_left: u32 = offsets[0].parse().unwrap();
        let in_from_top: u32 = offsets[1].parse().unwrap();
        let width: u32 = side_lens[0].parse().unwrap();
        let height: u32 = side_lens[1].parse().unwrap();
        let rect = Rectangle {
            in_from_left,
            in_from_top,
            width,
            height,
        };
        let claim = Claim { id, rect };
        claims.push(claim);
    }
    claims
}

#[derive(Debug)]
struct Claim {
    id: u32,
    rect: Rectangle,
}

#[derive(Debug)]
struct Rectangle {
    in_from_left: u32,
    in_from_top: u32,
    width: u32,
    height: u32,
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
