use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut calibrations: HashMap<isize, Vec<i32>> = HashMap::new();
    for line in inp.lines() {
        let split: Vec<&str> = line.split(':').collect();
        let key: isize = split[0].parse().expect("key to parse to isize");
        let values: Vec<i32> = split[1]
            .trim()
            .split(' ')
            .map(|v| v.parse().expect("value to parse to i32"))
            .collect();
        calibrations.insert(key, values);
    }
    dbg!(calibrations);
    "".to_string()
}

pub fn part_two(_inp: &str) -> String {
    "".to_string()
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
