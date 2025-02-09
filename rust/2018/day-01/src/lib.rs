pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let frequencies: Vec<i32> = inp.lines().map(|x| x.parse().unwrap()).collect();
    let mut freq_tracker = 0;
    for freq in frequencies {
        freq_tracker += freq;
    }
    freq_tracker.to_string()
}

pub fn part_two(inp: &str) -> String {
    let frequencies: Vec<i32> = inp.lines().map(|x| x.parse().unwrap()).collect();
    let mut freq_tracker = 0;
    let mut seen_freq: Vec<i32> = vec![0];
    loop {
        for freq in &frequencies {
            freq_tracker += freq;
            if seen_freq.contains(&freq_tracker) {
                return freq_tracker.to_string();
            } else {
                seen_freq.push(freq_tracker);
            }
        }
    }
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
