use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let card_map = fill_card_map(inp);
    let mut running_total_card_scores = 0;
    for (_, card) in card_map {
        let mut card_score: Option<i32> = None;
        for card_num in card.card_nums {
            if card.winning_nums.contains(&card_num) {
                match card_score {
                    Some(score) => card_score = Some(score * 2),
                    None => card_score = Some(1),
                }
            }
        }
        if let Some(score) = card_score {
            running_total_card_scores += score;
        }
    }
    running_total_card_scores.to_string()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

fn fill_card_map(inp: &str) -> HashMap<i32, Card> {
    let mut card_map: HashMap<i32, Card> = HashMap::new();
    for card in inp.lines() {
        let card = card.replace("Card ", "");
        // grab id from front of string
        let v: Vec<&str> = card.trim().split([':', '|']).collect();
        let id: i32 = v[0].parse().unwrap();
        let winning_nums: Vec<i32> = v[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let card_nums: Vec<i32> = v[2]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        card_map.insert(
            id,
            Card {
                winning_nums,
                card_nums,
            },
        );
    }
    card_map
}

#[derive(Debug)]
struct Card {
    winning_nums: Vec<i32>,
    card_nums: Vec<i32>,
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
