use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut grid = [[0u32; 1000]; 1000];
    for line in lines {
        let cmd = parse_line(line);
        grid = mutate_grid(cmd, grid);
    }
    dbg!(count_lights(&grid));
}

fn mutate_grid(cmd: Command, grid: [[u32; 1000]; 1000]) -> [[u32; 1000]; 1000] {
    // perform command on all included lights in grid
    let mut grid = grid;
    let tlc_x: u32 = cmd.tlc.x;
    let tlc_y: u32 = cmd.tlc.y;
    let brc_x: u32 = cmd.brc.x;
    let brc_y: u32 = cmd.brc.y;
    if &cmd.command == "on" {
        // fill range with 1
        for x in tlc_x..=brc_x {
            for y in tlc_y..=brc_y {
                grid[x as usize][y as usize] = 1;
            }
        }
    } else if &cmd.command == "off" {
        // fill range with 0
        for x in tlc_x..=brc_x {
            for y in tlc_y..=brc_y {
                grid[x as usize][y as usize] = 0;
            }
        }
    } else {
        // toggle
        for x in tlc_x..=brc_x {
            for y in tlc_y..=brc_y {
                if grid[x as usize][y as usize] == 0 {
                    grid[x as usize][y as usize] = 1;
                } else {
                    grid[x as usize][y as usize] = 0;
                }
            }
        }
    }
    grid
}

fn count_lights(grid: &[[u32; 1000]; 1000]) -> u32 {
    let mut lights = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x as usize][y as usize] == 1 {
                lights += 1;
            }
        }
    }
    lights
}

fn parse_line(line: &str) -> Command {
    // parse line
    let mut split = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let raw_cmd = &line[..split].trim();
    let mut line = &line[split..];
    split = line.find(|c: char| c.is_ascii_alphabetic()).unwrap();
    let tlc = &line[..split].trim();
    line = &line[split..];
    split = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let brc = &line[split..];
    // parse command
    let cmd: &str;
    if raw_cmd.contains("on") {
        cmd = "on";
    } else if raw_cmd.contains("off") {
        cmd = "off";
    } else {
        cmd = "toggle";
    }
    // parse tlc
    let top: Vec<&str> = tlc.split(',').collect();
    let tlc_x: u32 = top[0].parse().unwrap();
    let tlc_y: u32 = top[1].parse().unwrap();
    // parse brc
    let bot: Vec<&str> = brc.split(',').collect();
    let brc_x: u32 = bot[0].parse().unwrap();
    let brc_y: u32 = bot[1].parse().unwrap();
    Command {
        command: cmd.to_string(),
        tlc: Point { x: tlc_x, y: tlc_y },
        brc: Point { x: brc_x, y: brc_y },
    }
}
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Command {
    command: String,
    tlc: Point,
    brc: Point,
}
