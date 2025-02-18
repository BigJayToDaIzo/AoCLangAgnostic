pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut left_arr: Vec<i32> = Vec::new();
    let mut right_arr: Vec<i32> = Vec::new();
    for line in inp.lines() {
        let split = line.split_once("   ");
        match split {
            Some((l, r)) => {
                left_arr.push(l.parse().unwrap());
                right_arr.push(r.parse().unwrap());
            }
            None => (),
        }
    }

    left_arr.sort();
    right_arr.sort();
    let mut dist_sum = 0;
    for (i, left) in left_arr.iter().enumerate() {
        dist_sum += (left - right_arr[i]).abs();
    }
    dist_sum.to_string()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "2344935");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
