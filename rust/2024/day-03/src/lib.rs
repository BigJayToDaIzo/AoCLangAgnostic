use regex::Regex;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let matches: Vec<&str> = re.find_iter(inp).map(|m| m.as_str()).collect();
    let mut running_total_of_products = 0;
    for m in matches {
        let x: Vec<i32> = m
            .strip_prefix("mul(")
            .expect("m to begin with mul(")
            .strip_suffix(")")
            .expect("m to end with )")
            .split(',')
            .map(|x| x.parse().expect("value to parse to i32"))
            .collect();
        running_total_of_products += x[0] * x[1];
    }
    running_total_of_products.to_string()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
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
