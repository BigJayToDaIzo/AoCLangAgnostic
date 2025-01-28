use std::fs;

fn main() {
    // read in input.txt
    let input = fs::read_to_string("../input.txt").unwrap();
    let lines = input.lines();
    let mut nice_words = 0;
    let mut nice_words_vec = Vec::new();
    // nice:
    //  must contan min 3 vowels
    //  must contain at least 1 letter that appears twice in a row
    //  may NOT contain 'ab', 'cd', 'pq', or 'xy' at all
    // naughty otherwise
    for line in lines {
        if contains_naughty_sub(line) {
            continue;
        }
        if !contains_min_vowels(line) {
            continue;
        }
        if !contains_double_letter(line) {
            continue;
        }
        nice_words += 1;
        nice_words_vec.push(line);
    }
    println!("{}", nice_words);
    // dbg!(nice_words_vec);
}

fn contains_double_letter(s: &str) -> bool {
    let mut clipper = s.to_string();
    while clipper.len() >= 2 {
        if clipper.chars().next() == clipper.chars().nth(1) {
            return true;
        }
        clipper = clipper[1..].to_string();
    }
    false
}

fn contains_min_vowels(s: &str) -> bool {
    let mut vowel_count = 0;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
    }
    vowel_count >= 3
}

fn contains_naughty_sub(s: &str) -> bool {
    let naughty_subs = ["ab", "cd", "pq", "xy"];
    for sub in naughty_subs {
        if s.contains(sub) {
            return true;
        }
    }
    false
}
