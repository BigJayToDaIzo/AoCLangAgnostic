use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut valid_passphrases = 0;
    for passphrase in inp.lines() {
        let words: Vec<String> = passphrase.split(" ").map(str::to_string).collect();
        let mut candidate = true;
        for w in 0..words.len() - 1 {
            let dupe_candidate = &words[w];
            let words_remaining = &words[w + 1..];
            if words_remaining.contains(dupe_candidate) {
                candidate = false;
                break;
            }
        }
        if candidate {
            valid_passphrases += 1;
        }
    }
    valid_passphrases.to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut valid_passphrases = 0;
    for passphrase in inp.lines() {
        let words: Vec<String> = passphrase.split(" ").map(str::to_string).collect();
        let mut candidate = true;
        for w in 0..words.len() - 1 {
            let mut dupe_candidate: Vec<char> = words[w].chars().collect();
            dupe_candidate.sort_by(|a, b| b.cmp(a));
            let words_remaining = &words[w + 1..];
            for word in words_remaining {
                let mut check_anagram: Vec<char> = word.chars().collect();
                check_anagram.sort_by(|a, b| b.cmp(a));
                if dupe_candidate == check_anagram {
                    candidate = false;
                    break;
                }
            }
        }
        if candidate {
            valid_passphrases += 1;
        }
    }
    valid_passphrases.to_string()
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
