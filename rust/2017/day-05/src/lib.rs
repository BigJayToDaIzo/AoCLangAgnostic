pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    // parse jumps into usize array (should be len() 1043)
    let mut instructions: [i32; 1043] = [0; 1043];
    let mut ins_index: usize = 0;
    let mut steps = 0;
    for (i, line) in inp.lines().enumerate() {
        instructions[i] = line.parse().unwrap();
    }
    loop {
        // set jump offset
        let jump_offset: i32 = instructions[ins_index];
        steps += 1;
        // if ins_index + offset goes out of bounds, return steps
        if ins_index as i32 + jump_offset >= instructions.len() as i32
            || ins_index as i32 + jump_offset < 0
        {
            break;
        }
        // increment value at index
        instructions[ins_index] += 1;
        // jump
        ins_index = (ins_index as i32 + jump_offset) as usize;
    }
    steps.to_string()
}

pub fn part_two(inp: &str) -> String {
    // parse jumps into usize array (should be len() 1043)
    let mut instructions: [i32; 1043] = [0; 1043];
    let mut ins_index: usize = 0;
    let mut steps = 0;
    for (i, line) in inp.lines().enumerate() {
        instructions[i] = line.parse().unwrap();
    }
    loop {
        // set jump offset
        let jump_offset: i32 = instructions[ins_index];
        steps += 1;
        // if ins_index + offset goes out of bounds, return steps
        if ins_index as i32 + jump_offset >= instructions.len() as i32
            || ins_index as i32 + jump_offset < 0
        {
            break;
        }
        // increment value at index
        if jump_offset >= 3 {
            instructions[ins_index] -= 1;
        } else {
            instructions[ins_index] += 1;
        }
        // jump
        ins_index = (ins_index as i32 + jump_offset) as usize;
    }
    steps.to_string()
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
