pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let inp = inp.trim();
    let char_arr: Vec<char> = inp.chars().collect();
    collapse_polymer(char_arr).to_string()
}

pub fn part_two(inp: &str) -> String {
    let inp = inp.trim();
    let mut shortest_polymer = usize::MAX;
    let alphabet: Vec<char> = ('A'..='Z').collect();
    let char_arr: Vec<char> = inp.chars().collect();
    for char in alphabet {
        let mut ca = char_arr.clone();
        ca.retain(|c| c != &char && c != &char.to_ascii_lowercase());
        let poly_len = collapse_polymer(ca);
        if poly_len < shortest_polymer {
            shortest_polymer = poly_len;
        }
    }
    shortest_polymer.to_string()
}

fn collapse_polymer(mut char_arr: Vec<char>) -> usize {
    let mut had_reaction_needs_rechecked = true;
    while had_reaction_needs_rechecked {
        had_reaction_needs_rechecked = false;
        for i in 0..char_arr.len() - 1 {
            if char_arr[i].is_uppercase() && char_arr[i + 1].is_uppercase() {
                continue;
            }
            if char_arr[i].is_lowercase() && char_arr[i + 1].is_lowercase() {
                continue;
            }
            if char_arr[i].is_uppercase()
                && char_arr[i + 1].is_lowercase()
                && char_arr[i] == char_arr[i + 1].to_ascii_uppercase()
            {
                char_arr.remove(i);
                char_arr.remove(i);
                had_reaction_needs_rechecked = true;
                break;
            }
            if char_arr[i].is_lowercase()
                && char_arr[i + 1].is_uppercase()
                && char_arr[i] == char_arr[i + 1].to_ascii_lowercase()
            {
                char_arr.remove(i);
                char_arr.remove(i);
                had_reaction_needs_rechecked = true;
                break;
            }
        }
    }
    char_arr.len()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "10496");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "5774");
    }
}
