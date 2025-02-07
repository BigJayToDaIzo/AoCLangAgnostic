pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut rt = 0;
    let v: Vec<char> = inp.trim_end().chars().collect();
    for i in 0..v.len() {
        if i == v.len() - 1 && v[i] == v[0] {
            rt += v[i].to_digit(10).unwrap();
        }
        if i != v.len() - 1 && v[i] == v[i + 1] {
            rt += v[i].to_digit(10).unwrap();
        }
    }
    rt.to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut rt = 0;
    let v: Vec<char> = inp.trim_end().chars().collect();
    for i in 0..v.len() {
        if v[i] == v[(i + v.len() / 2) % v.len()] {
            rt += v[i].to_digit(10).unwrap();
        }
    }
    rt.to_string()
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
