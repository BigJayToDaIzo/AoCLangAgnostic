use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut keycode = String::from("");
    let mut key = "5";
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => {
                    if "12459".contains(key) {
                        continue;
                    } else {
                        key = move_up(key);
                    }
                }
                'D' => {
                    if "59ACD".contains(key) {
                        continue;
                    } else {
                        key = move_down(key);
                    }
                }
                'L' => {
                    if "125AD".contains(key) {
                        continue;
                    } else {
                        key = move_left(key);
                    }
                }
                'R' => {
                    if "149CD".contains(key) {
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
        "2" => "3",
        "3" => "4",
        "5" => "6",
        "6" => "7",
        "7" => "8",
        "8" => "9",
        "A" => "B",
        "B" => "C",
        _ => k,
    }
}

fn move_left(k: &str) -> &str {
    match k {
        "3" => "2",
        "4" => "3",
        "6" => "5",
        "7" => "6",
        "8" => "7",
        "9" => "8",
        "B" => "A",
        "C" => "B",
        _ => k,
    }
}

fn move_up(k: &str) -> &str {
    match k {
        "3" => "1",
        "6" => "2",
        "7" => "3",
        "8" => "4",
        "A" => "6",
        "B" => "7",
        "C" => "8",
        "D" => "B",
        _ => k,
    }
}

fn move_down(k: &str) -> &str {
    match k {
        "1" => "3",
        "2" => "6",
        "3" => "7",
        "4" => "8",
        "6" => "A",
        "7" => "B",
        "8" => "C",
        "B" => "D",
        _ => k,
    }
}
