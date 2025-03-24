use std::{error::Error, io::Read};

use crate::VALID;

/// "pre-processes" the contents of a file
/// into a vector of valid Brainf*ck instructions.
pub fn process(contents: String) -> Vec<char> {
    contents.chars().filter(|c| VALID.contains(c)).collect()
}

pub fn interpret(contents: String) -> Result<[i32; 30000], Box<dyn Error>> {
    let instructions = process(contents);

    let mut mem = [0; 30000];
    let mut ptr = 0;
    let mut pc = 0;

    while pc < instructions.len() {
        match instructions[pc] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => mem[ptr] += 1,
            '-' => mem[ptr] -= 1,
            '.' => print!("{}", mem[ptr] as u8 as char),
            ':' => print!("{}", mem[ptr]),
            ',' => {
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer)?;
                mem[ptr] = buffer[0] as i32;
            },
            '[' => {
                if mem[ptr] == 0 {
                    let mut depth = 1;
                    while depth != 0 {
                        pc += 1;
                        if instructions[pc] == '[' {
                            depth += 1;
                        } else if instructions[pc] == ']' {
                            depth -= 1;
                        }
                    }
                }
            },
            ']' => {
                if mem[ptr] != 0 {
                    let mut depth = 1;
                    while depth != 0 {
                        pc -= 1;
                        if instructions[pc] == ']' {
                            depth += 1;
                        } else if instructions[pc] == '[' {
                            depth -= 1;
                        }
                    }
                }
            },
            _ => {}
        }
        pc += 1;
    }

    Ok(mem)
}
