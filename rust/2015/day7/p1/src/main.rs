use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut wires: Vec<Wire> = Vec::new();
    // pre-process input into instructions (and raw wire input)
    for line in input.lines() {
        dbg!(line);
        let instruction = line.split(" -> ").collect::<Vec<&str>>();
        // build Instruction and push into instructions Vec
        // then build wires, and push the signals through those wires
        // to build the next gates/wires
        let instr = instruction[0];
        let output = instruction[1].to_string();
        if instr.contains("AND") {
            let words = instr.split(" ").collect::<Vec<&str>>();
            let lhs = words[0].to_string();
            let rhs = words[2].to_string();
            instructions.push(Instruction::And {
                lhs,
                rhs,
                output,
                executed: false,
            });
        } else if instr.contains("OR") {
            let words = instr.split(" ").collect::<Vec<&str>>();
            let lhs = words[0].to_string();
            let rhs = words[2].to_string();
            instructions.push(Instruction::Or {
                lhs,
                rhs,
                output,
                executed: false,
            });
        } else if instr.contains("NOT") {
            let wire_to_not = instr.split(" ").collect::<Vec<&str>>()[1].to_string();
            instructions.push(Instruction::Not {
                wire_to_not,
                output,
                executed: false,
            });
        } else if instr.contains("LSHIFT") {
            let words = instr.split(" ").collect::<Vec<&str>>();
            let input = words[0].to_string();
            let bits_to_shift: u16 = words[2].parse().unwrap();
            instructions.push(Instruction::Lshift {
                input,
                bits_to_shift,
                output,
                executed: false,
            });
        } else if instr.contains("RSHIFT") {
            let words = instr.split(" ").collect::<Vec<&str>>();
            let input = words[0].to_string();
            let bits_to_shift: u16 = words[2].parse().unwrap();
            instructions.push(Instruction::Rshift {
                input,
                bits_to_shift,
                output,
                executed: false,
            });
        } else if instr.chars().next().unwrap().is_alphabetic() {
            instructions.push(Instruction::Wire2Wire {
                input: instr.to_string(),
                output: output.to_string(),
                executed: false,
            });
        } else {
            let w = Wire {
                name: output.to_string(),
                output: instr.parse().unwrap(),
            };
            wires.push(w);
        }
    }
    // parse Instructions untill all wires are defined
}

#[derive(PartialEq, Debug)]
struct Wire {
    name: String,
    output: u32,
}

#[derive(Debug)]
enum Instruction {
    And {
        lhs: String,
        rhs: String,
        output: String,
        executed: bool,
    },
    Or {
        lhs: String,
        rhs: String,
        output: String,
        executed: bool,
    },
    Rshift {
        input: String,
        bits_to_shift: u16,
        output: String,
        executed: bool,
    },
    Lshift {
        input: String,
        bits_to_shift: u16,
        output: String,
        executed: bool,
    },
    Not {
        wire_to_not: String,
        output: String,
        executed: bool,
    },
    Wire2Wire {
        input: String,
        output: String,
        executed: bool,
    },
    Wire {
        input: u16,
        output: String,
    },
}

struct Wire2Wire {
    input: String,
    output: String,
    executed: bool,
}

struct And {
    lhs: u16,
    rhs: u16,
    output: String,
    executed: bool,
}

struct Or {
    lhs: u16,
    rhs: u16,
    output: String,
    executed: bool,
}

struct Rshift {
    input: String,
    bits_to_shift: u16,
    output: String,
    executed: bool,
}

struct Lshift {
    input: String,
    bits_to_shift: u16,
    output: String,
    executed: bool,
}
struct Not {
    wire_to_negate: String,
    output: String,
    executed: bool,
}
