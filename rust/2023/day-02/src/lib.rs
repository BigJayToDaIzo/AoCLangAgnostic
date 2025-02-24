pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut sum_of_game_ids = 0;
    for game in inp.lines() {
        let split: Vec<&str> = game.split(":").collect();
        let game_id: i32 = split[0]
            .strip_prefix("Game ")
            .unwrap()
            .parse()
            .expect("parse to i32");
        let draws: Vec<&str> = split[1].split(";").collect();
        let (max_red, max_green, max_blue) = return_max_dice(&draws);
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            sum_of_game_ids += game_id;
        }
    }
    sum_of_game_ids.to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut sum_of_game_power = 0;
    for game in inp.lines() {
        let split: Vec<&str> = game.split(":").collect();
        let draws: Vec<&str> = split[1].split(";").collect();
        let (max_red, max_green, max_blue) = return_max_dice(&draws);
        sum_of_game_power += max_red * max_green * max_blue;
    }
    sum_of_game_power.to_string()
}

fn return_max_dice(v: &Vec<&str>) -> (i32, i32, i32) {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for draw in v {
        let dice: Vec<&str> = draw.split(',').collect();
        for die in dice {
            let die_split: Vec<&str> = die.split(' ').collect();
            let num_dice = die_split[1].parse().expect("parse to i32");
            match die_split[2] {
                "red" => {
                    if max_red < num_dice {
                        max_red = num_dice;
                    }
                }
                "green" => {
                    if max_green < num_dice {
                        max_green = num_dice;
                    }
                }
                "blue" => {
                    if max_blue < num_dice {
                        max_blue = num_dice;
                    }
                }
                _ => (),
            }
        }
    }
    (max_red, max_green, max_blue)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "2716");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "72227");
    }
}
