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

pub fn part_two(_inp: &str) -> String {
    "abc".to_string()
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
