pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut sum_calibration_vals = 0;
    for line in inp.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut first_digit_idx = 0;
        let mut last_digit_idx = 0;
        if let Some(fdi) = chars.iter().position(|&c| c.is_ascii_digit()) {
            first_digit_idx = fdi;
        };
        if let Some(ldi) = chars.iter().rposition(|&c| c.is_ascii_digit()) {
            last_digit_idx = ldi;
        }
        let mut calibration_val_string = String::new();
        calibration_val_string.push(chars[first_digit_idx]);
        calibration_val_string.push(chars[last_digit_idx]);
        let sum_val: i32 = calibration_val_string
            .parse()
            .expect("value to parse to i32");
        sum_calibration_vals += sum_val;
    }
    sum_calibration_vals.to_string()
}

pub fn part_two(inp: &str) -> String {
    let digits: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    let word_digits: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];
    let mut first_digit_idx = usize::MAX;
    let mut second_digit_idx = 0;
    let mut first_digit_opt: Option<i32> = None;
    let mut second_digit_opt: Option<i32> = None;

    for line in inp.lines() {
        for digit in &digits {
            // if digit is_ascii_digit()
            if let Some(idx) = line.find(digit) {
                if idx < first_digit_idx {
                    first_digit_idx = idx;
                }
                if idx > second_digit_idx {
                    second_digit_idx = idx;
                }
            }
        }
        for digit in &word_digits {
            if let Some(idx) = line.find(digit) {
                if idx < first_digit_idx {
                    first_digit_opt = parse_word_digit(digit);
                }
                if idx > second_digit_idx {
                    second_digit_opt = parse_word_digit(digit);
                }
            }
        }
        match first_digit_opt {
            Some(digit) => {
                // multiply by 10 and add to second digit
                dbg!(digit);
            }
            None => {
                // parse from line with idx, then multiply by 10
                // and add to second digit
            }
        }
        match second_digit_opt {
            Some(digit) => {
                // add to first digit
                dbg!(digit);
            }
            None => {
                // parse second from line with idx
                // add to first digit
            }
        }
        // concat digits and convert to i32
    }

    "abc".to_string()
}

fn parse_word_digit(digit: &str) -> Option<i32> {
    match digit {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "zero" => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
