pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let (l_arr_sorted, r_arr_sorted) = parse_input(inp);
    let mut dist_sum = 0;
    for (i, left) in l_arr_sorted.iter().enumerate() {
        dist_sum += (left - r_arr_sorted[i]).abs();
    }
    dist_sum.to_string()
}

pub fn part_two(inp: &str) -> String {
    let (l_arr_sorted, r_arr_sorted) = parse_input(inp);
    let mut sim_score = 0;
    for left in &l_arr_sorted {
        let mut sim_multi = 0;
        for right in &r_arr_sorted {
            if left == right {
                sim_multi += 1;
            }
        }
        sim_score += sim_multi * left;
    }
    sim_score.to_string()
}

fn parse_input(inp: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l_arr: Vec<i32> = Vec::new();
    let mut r_arr: Vec<i32> = Vec::new();
    for line in inp.lines() {
        let split = line.split_once("   ");
        if let Some((l, r)) = split {
            l_arr.push(l.parse().unwrap());
            r_arr.push(r.parse().unwrap());
        };
    }
    l_arr.sort();
    r_arr.sort();
    (l_arr, r_arr)
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

        assert_eq!(res, "27647262");
    }
}
