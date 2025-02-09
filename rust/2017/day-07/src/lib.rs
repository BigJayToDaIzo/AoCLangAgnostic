use std::cell::RefCell;
use std::collections::HashMap;

pub fn read_lines() -> String {
    include_str!("../input.txt").to_owned()
}

pub fn part_one(inp: &str) -> String {
    // parse lines into programs
    // weight seems irrelevant for this part?
    let mut program_tower: HashMap<String, RefCell<Program>> = HashMap::new();
    for line in inp.lines() {
        let mut l: Vec<&str> = line.split_whitespace().collect();
        let name = l[0].to_string();
        let weight: u32 = l[1]
            .to_string()
            .replace("(", "")
            .replace(")", "")
            .parse()
            .unwrap();
        let mut sub_progs: Vec<String> = vec![];

        if l.len() > 2 {
            let l = l.split_off(3);
            for prog in l {
                sub_progs.push(prog.to_string());
            }
            program_tower.insert(
                name,
                RefCell::new(Program {
                    weight,
                    has_parent: false,
                    sub_progs: Some(sub_progs),
                }),
            );
        } else {
            program_tower.insert(
                name,
                RefCell::new(Program {
                    weight,
                    has_parent: false,
                    sub_progs: None,
                }),
            );
        }
    }
    // out of order program tower complete
    for (_, program) in &program_tower {
        match program.borrow_mut().sub_progs {
            Some(sub_progs) => {
                for sub in sub_progs {
                    match program_tower.get(&sub) {
                        Some(p) => {
                            p.borrow_mut().has_parent = true;
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }
    // find the root
    let mut root = String::new();
    root.to_string()
}

pub fn part_two(inp: &str) -> String {
    "".to_string()
}

#[derive(Debug)]
struct Program {
    weight: u32,
    has_parent: bool,
    sub_progs: Option<Vec<String>>,
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
