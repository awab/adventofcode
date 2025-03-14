use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone)]
enum Instruction {
    snd(char),
    set(char, i128),
    set_register(char, char),
    add(char, i128),
    add_register(char, char),
    mul(char, i128),
    mul_register(char, char),
    modulo(char, i128),
    modulo_register(char, char),
    rcv(char),
    jgz(char, i128),
    jgz_value(i128, i128),
    jgz_register(char, char),
}

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut program = vec![];
    let mut registers: HashMap<char, i128> = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "snd" => {
                program.push(Instruction::snd(parts[1].chars().nth(0).unwrap()));
            },
            "set" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::set(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::set_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "add" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::add(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::add_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
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
            "mod" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::modulo(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::modulo_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "rcv" => {
                program.push(Instruction::rcv(parts[1].chars().nth(0).unwrap()));
            },
            "jgz" => {
                let parse_check = parts[1].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::jgz_value(parse_check.unwrap(), parts[2].parse::<i128>().unwrap()));
                } else {
                    let parse_check = parts[2].parse::<i128>();
                    if parse_check.is_ok() {
                        program.push(Instruction::jgz(parts[1].chars().nth(0).unwrap(), parts[2].parse::<i128>().unwrap()));
                    } else {
                        program.push(Instruction::jgz_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                    }
                }
            },
            _ => panic!("Unknown command found {}", parts[0]),
        };
    }

    let mut pc: usize = 0;
    let mut signals = HashMap::new();
    loop {
        if pc >= program.len() {
            return;
        }

        let instruction = &program[pc];
        match instruction {
            Instruction::snd(register) => {
                let value = registers.entry(*register).or_insert(0i128);
                signals.insert(*register, *value);

                pc += 1;
            },
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
            Instruction::add(target, value) => {
                let value_check = registers.get(target);
                let mut new_value = 0;
                if value_check != None {
                    new_value = *value_check.unwrap();
                }
                new_value += value;
                registers.insert(*target, new_value);

                pc += 1;
            },
            Instruction::add_register(target, register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let value_check = registers.get(target);
                if value_check != None {
                    value += *value_check.unwrap();
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
            },
            Instruction::modulo(target, new_value) => {
                let value_check = registers.get(target);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }
                let new_value = value % *new_value;
                registers.insert(*target, new_value);

                pc += 1;
            },
            Instruction::modulo_register(target, register) => {
                let value_check = registers.get(target);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let value_check = registers.get(register);
                if value_check != None {
                    value %= *value_check.unwrap();
                } else {
                    panic!("Problem!");
                }

                registers.insert(*target, value);

                pc += 1;
            },
            Instruction::rcv(register) => {
                let value_check = signals.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                } else {
                    pc += 1;
                    continue;
                }
                
                println!("Signal received: {}", value);
                return;
                
                pc += 1;
            },
            Instruction::jgz(register, offset) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value > 0 {
                    pc = ((pc as i128) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jgz_value(value, offset) => {
                if *value > 0 {
                    pc = ((pc as i128) + *offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jgz_register(register, offset_register) => {
                let value_check = registers.get(register);
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value > 0 {
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
}
*/


fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut program = vec![];
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[0] {
            "snd" => {
                program.push(Instruction::snd(parts[1].chars().nth(0).unwrap()));
            },
            "set" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::set(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::set_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "add" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::add(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::add_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
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
            "mod" => {
                let parse_check = parts[2].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::modulo(parts[1].chars().nth(0).unwrap(), parse_check.unwrap()));
                } else {
                    program.push(Instruction::modulo_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                }
            },
            "rcv" => {
                program.push(Instruction::rcv(parts[1].chars().nth(0).unwrap()));
            },
            "jgz" => {
                let parse_check = parts[1].parse::<i128>();
                if parse_check.is_ok() {
                    program.push(Instruction::jgz_value(parse_check.unwrap(), parts[2].parse::<i128>().unwrap()));
                } else {
                    let parse_check = parts[2].parse::<i128>();
                    if parse_check.is_ok() {
                        program.push(Instruction::jgz(parts[1].chars().nth(0).unwrap(), parts[2].parse::<i128>().unwrap()));
                    } else {
                        program.push(Instruction::jgz_register(parts[1].chars().nth(0).unwrap(), parts[2].chars().nth(0).unwrap()));
                    }
                }
            },
            _ => panic!("Unknown command found {}", parts[0]),
        };
    }

    let mut pc1: usize = 0;
    let mut pc2: usize = 0;
    let mut program2_sends = 0;
    let mut program1_done = false;
    let mut program2_done = false;
    let mut waiting_for_signal1 = '\0';
    let mut waiting_for_signal2 = '\0';
    let mut signals1 = vec![];
    let mut signals2 = vec![];
    let mut registers1: HashMap<char, i128> = HashMap::new();
    let mut registers2: HashMap<char, i128> = HashMap::new();
    
    registers1.insert('p', 0);
    registers2.insert('p', 1);

    let mut index = 0;
    loop {
        if !program1_done && pc1 >= program.len() {
            program1_done = true;
        }

        if !program2_done && pc2 >= program.len() {
            program2_done = true;
        }

        if waiting_for_signal1 != '\0' && waiting_for_signal2 != '\0' ||
         program1_done && waiting_for_signal2 != '\0' ||
         program2_done && waiting_for_signal1 != '\0' {
            // deadlock
            break;
        }

        let mut pc = pc1;
        if index % 2 != 0 {
            pc = pc2;
        }

        let mut instruction = &program[pc];
        if index % 2 == 0 && waiting_for_signal1 != '\0' {
            if signals2.len() > 0 {
                let signal = signals2[0];
                signals2.remove(0);
                registers1.insert(waiting_for_signal1, signal);
                waiting_for_signal1 = '\0';
            } else {
                index += 1;
                continue;
            }
        }

        if index % 2 != 0 && waiting_for_signal2 != '\0' {
            if signals1.len() > 0 {
                let signal = signals1[0];
                signals1.remove(0);
                registers2.insert(waiting_for_signal2, signal);
                waiting_for_signal2 = '\0';
            } else {
                index += 1;
                continue;
            }
        }

        match instruction {
            Instruction::snd(register) => {
                let mut value = 0;
                if index % 2 == 0 {
                    value = *registers1.entry(*register).or_insert(0i128);
                    signals1.push(value);
                } else {
                    value = *registers2.entry(*register).or_insert(0i128);
                    signals2.push(value);
                    program2_sends += 1;
                }

                pc += 1;
            },
            Instruction::set(target, source) => {
                if index % 2 == 0 {
                    registers1.insert(*target, *source);
                } else {
                    registers2.insert(*target, *source);
                }

                pc += 1;
            },
            Instruction::set_register(target, register) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if index % 2 == 0 {
                    registers1.insert(*target, value);
                } else {
                    registers2.insert(*target, value);
                }

                pc += 1;
            },
            Instruction::add(target, value) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                }
                let mut new_value = 0;
                if value_check != None {
                    new_value = *value_check.unwrap();
                }
                new_value += value;
                
                if index % 2 == 0 {
                    registers1.insert(*target, new_value);
                } else {
                    registers2.insert(*target, new_value);
                }

                pc += 1;
            },
            Instruction::add_register(target, register) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                }
                if value_check != None {
                    value += *value_check.unwrap();
                }

                if index % 2 == 0 {
                    registers1.insert(*target, value);
                } else {
                    registers2.insert(*target, value);
                }

                pc += 1;
            },
            Instruction::mul(target, value) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                }

                let mut new_value = 0;
                if value_check != None {
                    new_value = *value_check.unwrap();
                }
                new_value *= value;
                if index % 2 == 0 {
                    registers1.insert(*target, new_value);
                } else {
                    registers2.insert(*target, new_value);
                }

                pc += 1;
            },
            Instruction::mul_register(target, register) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                };
                if value_check != None {
                    value *= *value_check.unwrap();
                } else {
                    value = 0;
                }

                if index % 2 == 0 {
                    registers1.insert(*target, value);
                } else {
                    registers2.insert(*target, value);
                }

                pc += 1;
            },
            Instruction::modulo(target, new_value) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }
                let new_value = value % *new_value;
                
                if index % 2 == 0 {
                    registers1.insert(*target, new_value);
                } else {
                    registers2.insert(*target, new_value);
                }

                pc += 1;
            },
            Instruction::modulo_register(target, register) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(target);
                } else {
                    value_check = registers2.get(target);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }
                if value_check != None {
                    value %= *value_check.unwrap();
                } else {
                    panic!("Problem!");
                }

                if index % 2 == 0 {
                    registers1.insert(*target, value);
                } else {
                    registers2.insert(*target, value);
                }

                pc += 1;
            },
            Instruction::rcv(register) => {
                if index % 2 == 0 {
                    // program1, use signals2
                    if signals2.len() > 0 {
                        let signal = signals2[0];
                        signals2.remove(0);
                        registers1.insert(*register, signal);
                        waiting_for_signal1 = '\0';
                    } else {
                        waiting_for_signal1 = *register;
                    }
                } else {
                    // program2, use signals1
                    if signals1.len() > 0 {
                        let signal = signals1[0];
                        signals1.remove(0);
                        registers2.insert(*register, signal);
                        waiting_for_signal1 = '\0';
                    } else {
                        waiting_for_signal2 = *register;
                    } 
                }
                
                pc += 1;
            },
            Instruction::jgz(register, offset) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }
                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value > 0 {
                    pc = ((pc as i128) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jgz_value(value, offset) => {
                if *value > 0 {
                    pc = ((pc as i128) + *offset) as usize;
                } else {
                    pc += 1;
                }
            },
            Instruction::jgz_register(register, offset_register) => {
                let mut value_check = None;
                if index % 2 == 0 {
                    value_check = registers1.get(register);
                } else {
                    value_check = registers2.get(register);
                }

                let mut value = 0;
                if value_check != None {
                    value = *value_check.unwrap();
                }

                if value > 0 {
                    let mut value_check = None;
                    if index % 2 == 0 {
                        value_check = registers1.get(offset_register);
                    } else {
                        value_check = registers2.get(offset_register);
                    }
                    let mut offset = 0;
                    if value_check != None {
                        offset = *value_check.unwrap();
                    }

                    pc = ((pc as i128) + offset) as usize;
                } else {
                    pc += 1;
                }
            },
            _ => panic!("Unknown instruction!"),
        }

        if index % 2 != 0 {
            pc2 = pc;
        } else {
            pc1 = pc;
        }

        index += 1;
    }

    println!("Program 1 sent {} signals", program2_sends);
}