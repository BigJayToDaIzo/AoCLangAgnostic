use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut exactly_twos = 0;
    let mut exactly_threes = 0;
    for line in inp.lines() {
        let mut box_id_map: HashMap<char, u32> = HashMap::new();
        let box_chars: Vec<char> = line.chars().collect();
        for char in box_chars {
            let count = box_id_map.entry(char).or_insert(0);
            *count += 1;
        }
        for count in box_id_map.values() {
            //exactly twos
            if *count == 2 {
                exactly_twos += 1;
                break;
            }
        }
        for count in box_id_map.values() {
            //exactly threes
            if *count == 3 {
                exactly_threes += 1;
                break;
            }
        }
    }
    (exactly_twos * exactly_threes).to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut lines: Vec<&str> = inp.lines().collect();
    let mut shrinking_lines = lines.clone();
    loop {
        let comparator = shrinking_lines.pop();
        if let Some(cmp) = comparator {
            lines.retain(|v| *v != cmp);
            // then iterate over remaining lines
            for line in &lines {
                let cmp_iter = cmp.chars();
                let line_char_vec: Vec<char> = line.chars().collect();
                let mut num_delta = 0;
                let mut rogue_char = ' ';
                for (i, c) in cmp_iter.enumerate() {
                    if c != line_char_vec[i] {
                        num_delta += 1;
                        if num_delta > 1 {
                            break;
                        }
                        rogue_char = c;
                    }
                }
                if num_delta == 1 {
                    return line.replace(rogue_char, "");
                }
            }
        };
    }
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
