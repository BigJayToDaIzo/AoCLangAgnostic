pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let mut checksum = 0;
    for line in inp.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();
        checksum += max - min;
    }
    checksum.to_string()
}

pub fn part_two(inp: &str) -> String {
    let mut checksum = 0;
    for line in inp.lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] % nums[j] == 0 {
                    println!(
                        "nums[i]: {} divides evenly into nums[j]: {} = {}",
                        nums[i],
                        nums[j],
                        nums[i] / nums[j]
                    );
                    checksum += nums[i] / nums[j];
                } else if nums[j] % nums[i] == 0 {
                    println!(
                        "nums[j]: {} divides evenly into nums[i]: {} = {}",
                        nums[j],
                        nums[i],
                        nums[j] / nums[i]
                    );
                    checksum += nums[j] / nums[i];
                }
            }
        }
    }
    checksum.to_string()
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
