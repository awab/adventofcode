use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone)]
enum Instruction {
    set(char, i128),
    set_register(char, char),
    sub(char, i128),
    sub_register(char, char),
    mul(char, i128),
    mul_register(char, char),
    jnz(char, i128),
    jnz_value(i128, i128),
    jnz_register(char, char),
}

/* Part 1

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut program = vec![];
    let mut registers: HashMap<char, i128> = HashMap::new();
    registers.insert('a', 1);
    registers.insert('b', 0);
    registers.insert('c', 0);
    registers.insert('d', 0);
    registers.insert('e', 0);
    registers.insert('f', 0);
    registers.insert('g', 0);
    registers.insert('h', 0);

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "set" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::set(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::set_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "sub" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::sub(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::sub_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "mul" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::mul(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::mul_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "jnz" => {
                let parse_check = parts[1].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::jnz_value(parse_check.unwrap(), parts[2].parse::<i128>().unwrap()));
                } else {
                    let parse_check = parts[2].parse::<i128>();
                    if parse_check.is_ok() {
                        program.push(Instruction::jnz(parts[1].chars().nth(0).unwrap(), parts[2].parse::<i128>().unwrap()));
                    } else {
                        program.push(Instruction::jnz_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                    }
                }
            },
            _ => panic!("Unknown command found {}", parts[0]),
        };
    }

    let mut pc: usize = 0;
    // Part 1: let mut mul_count = 0;
    
    loop {
        if pc >= program.len() {
            break;
        }

        let instruction = &program[pc];
        match instruction {
            Instruction::set(target, source) => {
                registers.insert(*target, *source);

                pc += 1;
            },
            Instruction::set_register(target, register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                registers.insert(*target, value);

                pc += 1;
            },
            Instruction::sub(target, value) => {
                let value_check = registers.get(target);
                let mut new_value = 0;
                if value_check != None {
                    new_value = *value_check.unwrap();
                }
                new_value -= value;
                registers.insert(*target, new_value);

                pc += 1;
            },
            Instruction::sub_register(target, register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let value_check = registers.get(target);
                if value_check != None {
                    value -= *value_check.unwrap();
                }

                registers.insert(*target, value);

                pc += 1;
            },
            Instruction::mul(target, value) => {
                let value_check = registers.get(target);
                let mut new_value = 0;
                if value_check != None {
                    new_value = *value_check.unwrap();
                }
                new_value *= value;
                registers.insert(*target, new_value);

                pc += 1;
                // Part 1: mul_count += 1;
            },
            Instruction::mul_register(target, register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let value_check = registers.get(target);
                if value_check != None {
                    value *= *value_check.unwrap();
                } else {
                    value = 0;
                }

                registers.insert(*target, value);

                pc += 1;
                // Part 1: mul_count += 1;
            },
            Instruction::jnz(register, offset) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value != 0 {
                    pc = ((pc as i128) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jnz_value(value, offset) => {
                if *value != 0 {
                    pc = ((pc as i128) + *offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jnz_register(register, offset_register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value != 0 {
                    let value_check = registers.get(offset_register);
                    let mut offset = 0;
                    if value_check != None {
                        offset = *value_check.unwrap();
                    }

                    pc = ((pc as i128) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            _ => continue,
        }
    }

    println!("Register h: {}", registers.get(&'h').unwrap());
    // Part 1: println!("Total mul count: {}", mul_count);
}

*/

fn main() {
    // the program is calculating the two numbers, stepping by 17, and counting non-primes
    let mut b = (81 * 100) + 100000;
    let mut c = b + 17000;
    let mut h = 0;

    'number: for number in (b..=c).step_by(17) {
        for i in 2..(number / 2) {
            if number % i == 0 {
                h += 1;
                continue 'number;
            }
        }
    }

    println!("Register h: {}", h);
}