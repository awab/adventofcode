use std::fs::read_to_string;

#[derive(Clone)]
enum Instruction {
    NOP,
    HLF(char),
    TPL(char),
    INC(char),
    JMP(i16),
    JIE(char, i16),
    JIO(char, i16),
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut a: u128 = 1;
    let mut b: u128 = 0;
    let mut pc: usize = 0;
    let mut instructions = vec![];

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let mut instruction = Instruction::NOP;
        match parts[0] {
            "hlf" => {
                instruction = Instruction::HLF(parts[1].chars().nth(0).unwrap());
            },
            "tpl" => {
                instruction = Instruction::TPL(parts[1].chars().nth(0).unwrap());
            },
            "inc" => {
                instruction = Instruction::INC(parts[1].chars().nth(0).unwrap());
            },
            "jmp" => {
                instruction = Instruction::JMP(parse_digit(parts[1]));
            },
            "jie" => {
                instruction = Instruction::JIE(parts[1].chars().nth(0).unwrap(), parse_digit(parts[2]));
            },
            "jio" => {
                instruction = Instruction::JIO(parts[1].chars().nth(0).unwrap(), parse_digit(parts[2]));
            }
            _ => panic!("Unknown instruction!"),
        };

        instructions.push(instruction);
    }

    while pc < instructions.len() {
        let instruction = instructions[pc].clone();
        
        match instruction {
            Instruction::NOP => {

            },
            Instruction::HLF(register) => {
                if register == 'a' {
                    a /= 2;
                } else if register == 'b' {
                    b /= 2;
                }

                pc += 1;
            },
            Instruction::TPL(register) => {
                if register == 'a' {
                    a *= 3;
                } else if register == 'b' {
                    b *= 3;
                }

                pc += 1;
            },
            Instruction::INC(register) => {
                if register == 'a' {
                    a += 1;
                } else if register == 'b' {
                    b += 1;
                }

                pc += 1;
            },
            Instruction::JMP(offset) => {
                pc = ((pc as i16) + offset) as usize;
            },
            Instruction::JIE(register, offset) => {
                let mut value = a;
                if register == 'b' {
                    value = b;
                }

                if value % 2 == 0 {
                    pc = ((pc as i16) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::JIO(register, offset) => {
                let mut value = a;
                if register == 'b' {
                    value = b;
                }

                if value == 1 {
                    pc = ((pc as i16) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
        }
    }

    println!("Final register b value: {}", b);
}

fn parse_digit(input: &str) -> i16 {
    input.parse::<i16>().unwrap()
}
