pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    let (p_o_r_vec, updates_vec) = parse_input(inp);
    let mut good_update_middle_page_nums = Vec::new();
    'upd: for update in &updates_vec {
        for rule in &p_o_r_vec {
            if update.contains(&rule.before)
                && update.contains(&rule.after)
                && !update_in_order(update.to_vec(), &rule.before, &rule.after)
            {
                continue 'upd;
            }
        }
        good_update_middle_page_nums.push(update[update.len() / 2]);
    }
    let mpn_sums: u32 = good_update_middle_page_nums.iter().sum();
    mpn_sums.to_string()
}

pub fn part_two(inp: &str) -> String {
    let (p_o_r_vec, updates_vec) = parse_input(inp);
    // add pushing out of order updates into an array
    for update in updates_vec {
        for rule in &p_o_r_vec {
            if update.contains(&rule.before)
                && update.contains(&rule.after)
                && !update_in_order(update.to_vec(), &rule.before, &rule.after)
            {
                // run update through rules and FIX it
                // push middle page num of fixed update into
                // fixed_bad_update_middle_page_nums array
            }
        }
    }
    let fixed_bad_update_middle_page_nums = Vec::new();
    // sum middle pages
    let ooo_mpn_sums: u32 = fixed_bad_update_middle_page_nums.iter().sum();
    ooo_mpn_sums.to_string()
}

fn parse_input(inp: &str) -> (Vec<OrderingRule>, Vec<Vec<u32>>) {
    let split_input: Vec<&str> = inp.split_terminator("\n\n").collect();
    let page_ordering_rules = split_input[0];
    let updates = split_input[1];
    let mut p_o_r_vec: Vec<OrderingRule> = Vec::new();
    for rule in page_ordering_rules.lines() {
        let rule_split: Vec<u32> = rule
            .split("|")
            .map(|r| r.parse().expect("value to parse into u32"))
            .collect();
        p_o_r_vec.push(OrderingRule {
            before: rule_split[0],
            after: rule_split[1],
        });
    }
    let mut updates_vec: Vec<Vec<u32>> = Vec::new();
    for update in updates.lines() {
        let update_split: Vec<u32> = update
            .split(',')
            .map(|u| u.parse().expect("value to parse into u32"))
            .collect();
        updates_vec.push(update_split);
    }
    (p_o_r_vec, updates_vec)
}

fn update_in_order(upd: Vec<u32>, before: &u32, after: &u32) -> bool {
    let i_b = upd.iter().position(|x| x == before).unwrap();
    let i_a = upd.iter().position(|x| x == after).unwrap();
    i_b < i_a
}

#[derive(Debug, Copy, Clone)]
struct OrderingRule {
    before: u32,
    after: u32,
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn test_part_one() {
        let input = read_lines();

        let res = part_one(&input);

        assert_eq!(res, "7024");
    }

    #[test]
    pub fn test_part_two() {
        let input = read_lines();

        let res = part_two(&input);

        assert_eq!(res, "");
    }
}
