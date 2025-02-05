use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut keycode = String::from("");
    let mut key = "5";
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if "123".contains(key) {
                        continue;
                    } else {
                        key = move_up(key);
                    }
                }
                'D' => {
                    if "789".contains(key) {
                        continue;
                    } else {
                        key = move_down(key);
                    }
                }
                'L' => {
                    if "147".contains(key) {
                        continue;
                    } else {
                        key = move_left(key);
                    }
                }
                'R' => {
                    if "369".contains(key) {
                        continue;
                    } else {
                        key = move_right(key);
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        keycode += key;
    }
    dbg!(keycode);
}

fn move_right(k: &str) -> &str {
    match k {
        "1" => "2",
        "2" => "3",
        "4" => "5",
        "5" => "6",
        "7" => "8",
        "8" => "9",
        _ => k,
    }
}

fn move_left(k: &str) -> &str {
    match k {
        "2" => "1",
        "3" => "2",
        "5" => "4",
        "6" => "5",
        "8" => "7",
        "9" => "8",
        _ => k,
    }
}

fn move_up(k: &str) -> &str {
    match k {
        "4" => "1",
        "5" => "2",
        "6" => "3",
        "7" => "4",
        "8" => "5",
        "9" => "6",
        _ => k,
    }
}

fn move_down(k: &str) -> &str {
    match k {
        "1" => "4",
        "2" => "5",
        "3" => "6",
        "4" => "7",
        "5" => "8",
        "6" => "9",
        _ => k,
    }
}
