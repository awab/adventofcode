use std::fs::read_to_string;

struct Registers {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 1, // Part 1: 0
        d: 0,
    };

    let mut pc: usize = 0;
    let mut program = vec![];
    for line in lines {
        program.push(line);
    }

    while pc < program.len() {
        let current_line = &program[pc];
        let parts: Vec<_> = current_line.trim().split_whitespace().collect();
        
        match parts[0] {
            "cpy" => {
                let mut from_value = 0;

                match parts[1] {
                    "a" => from_value = registers.a,
                    "b" => from_value = registers.b,
                    "c" => from_value = registers.c,
                    "d" => from_value = registers.d,
                    _ => {
                        // we assume it's an integer
                        from_value = parts[1].parse::<i32>().unwrap();
                    }
                };

                match parts[2] {
                    "a" => registers.a = from_value,
                    "b" => registers.b = from_value,
                    "c" => registers.c = from_value,
                    "d" => registers.d = from_value,
                    _ => panic!("Unknown cpy destination!"),
                };

                pc += 1;
            },
            "inc" => {
                match parts[1] {
                    "a" => registers.a += 1,
                    "b" => registers.b += 1,
                    "c" => registers.c += 1,
                    "d" => registers.d += 1,
                    _ => panic!("Unknown inc destination!"),
                };

                pc += 1;
            },
            "dec" => {
                match parts[1] {
                    "a" => registers.a -= 1,
                    "b" => registers.b -= 1,
                    "c" => registers.c -= 1,
                    "d" => registers.d -= 1,
                    _ => panic!("Unknown dec destination!"),
                };

                pc += 1;
            },
            "jnz" => {
                let mut check_value = 0;
                match parts[1] {
                    "a" => check_value = registers.a,
                    "b" => check_value = registers.b,
                    "c" => check_value = registers.c,
                    "d" => check_value = registers.d,
                    _ => {
                        // we assume it's an integer
                        check_value = parts[1].parse::<i32>().unwrap();
                    }
                };

                if check_value != 0 {
                    pc = ((pc as i32) + parts[2].parse::<i32>().unwrap()) as usize;
                } else {
                    pc += 1;
                }
            },
            _ => panic!("Unknown instruction!"),
        };
    }

    println!("Register a: {}", registers.a);
}
