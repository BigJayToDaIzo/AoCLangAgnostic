pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut safe_lines = 0;
    for line in inp.lines() {
        let report: Vec<i32> = build_report(line); // all asc or desc?
        if is_ascending(&report) || is_descending(&report) {
            safe_lines += 1;
        }
    }
    safe_lines.to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut safe_lines = 0;
    for line in inp.lines() {
        let mut report: Vec<i32> = build_report(line);
        if is_asc_in_tolerance(&mut report) || is_desc_in_tolerance(&mut report) {
            safe_lines += 1;
        }
    }
    safe_lines.to_string()
}

fn build_report(lines: &str) -> Vec<i32> {
    let report: Vec<i32> = lines
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect();
    report
}

fn is_asc_in_tolerance(report: &mut Vec<i32>) -> bool {
    for (i, v) in report.iter().enumerate() {
        if i != report.len() - 1 {
            if v >= &report[i + 1] {
                report.remove(i);
                return is_ascending(report);
            }
            let diff = report[i + 1] - v;
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    }
    true
}

fn is_desc_in_tolerance(report: &mut Vec<i32>) -> bool {
    for (i, v) in report.iter().enumerate() {
        if i != report.len() - 1 {
            if v <= &report[i + 1] {
                report.remove(i);
                return is_descending(report);
            }
            let diff = v - report[i + 1];
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    }
    true
}

fn is_ascending(report: &[i32]) -> bool {
    for (i, v) in report.iter().enumerate() {
        if i != report.len() - 1 {
            if v >= &report[i + 1] {
                return false;
            }
            let diff = report[i + 1] - v;
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    }
    true
}

fn is_descending(report: &[i32]) -> bool {
    for (i, v) in report.iter().enumerate() {
        if i != report.len() - 1 {
            if v <= &report[i + 1] {
                return false;
            }
            let diff = v - report[i + 1];
            if !(1..=3).contains(&diff) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "334");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "400");
    }
}
