use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut nice_words = 0;
    for line in lines {
        if pair_appears_twice(line) && has_letter_repeated_with_one_letter_in_between(line) {
            nice_words += 1;
        }
    }
    dbg!(nice_words);
}

fn pair_appears_twice(s: &str) -> bool {
    let mut clipper = s.to_string();
    while clipper.len() >= 4 {
        let pre = &clipper[..2];
        let post = &clipper[2..];
        if post.contains(pre) {
            return true;
        }
        clipper = clipper[1..].to_string();
    }
    false
}

fn has_letter_repeated_with_one_letter_in_between(s: &str) -> bool {
    let mut clipper = s.to_string();
    while clipper.len() >= 3 {
        let letter = clipper.chars().next();
        if letter != clipper.chars().nth(1) && letter == clipper.chars().nth(2) {
            return true;
        }
        clipper = clipper[1..].to_string();
    }
    false
}
