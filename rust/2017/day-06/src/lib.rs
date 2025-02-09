pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    // scan banks into array, add array to seen_bank_configs
    let mut seen_bank_configs: Vec<Vec<u32>> = Vec::new();
    let mem_bank_len = 16;
    let mut mem_bank: Vec<u32> = inp
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    seen_bank_configs.push(mem_bank.clone());
    let mut seen_banks = 1;

    // loop until seen_bank_configs contains bank_config post realloc
    loop {
        // reallocate
        let mut max: u32 = 0;
        let mut max_index: usize = 0;
        // find INDEX of max block AND value (for range on redistribution)
        for (i, bank) in mem_bank.iter().enumerate() {
            if max < *bank {
                max = *bank;
                max_index = i;
            }
        }
        // set range to reallocate blocks
        mem_bank[max_index] = 0;
        for realloc in 1..=max {
            mem_bank[(max_index + realloc as usize) % mem_bank_len] += 1;
        }
        // check reallocation array
        if seen_bank_configs.contains(&mem_bank) {
            break;
        } else {
            seen_bank_configs.push(mem_bank.clone());
        }
        seen_banks += 1;
    }
    seen_banks.to_string()
}

pub fn part_two(inp: &str) -> String {
    // scan banks into array, add array to seen_bank_configs
    let mut seen_bank_configs: Vec<Vec<u32>> = Vec::new();
    let mem_bank_len = 16;
    let mut mem_bank: Vec<u32> = inp
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    seen_bank_configs.push(mem_bank.clone());
    let mut _seen_banks = 1;
    let magic_bank: usize;

    // loop until seen_bank_configs contains bank_config post realloc
    loop {
        // reallocate
        let mut max: u32 = 0;
        let mut max_index: usize = 0;
        // find INDEX of max block AND value (for range on redistribution)
        for (i, bank) in mem_bank.iter().enumerate() {
            if max < *bank {
                max = *bank;
                max_index = i;
            }
        }
        // set range to reallocate blocks
        mem_bank[max_index] = 0;
        for realloc in 1..=max {
            mem_bank[(max_index + realloc as usize) % mem_bank_len] += 1;
        }
        // check reallocation array
        if seen_bank_configs.contains(&mem_bank) {
            let bank = mem_bank.clone();
            magic_bank = seen_bank_configs.iter().position(|x| x == &bank).unwrap();
            break;
        } else {
            seen_bank_configs.push(mem_bank.clone());
        }
        _seen_banks += 1;
    }
    (seen_bank_configs.len() as u32 - magic_bank as u32).to_string()
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
