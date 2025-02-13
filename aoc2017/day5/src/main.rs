use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut program = vec![];
    let mut steps = 0;
    let mut pc = 0;

    for line in lines {
        program.push(line.parse::<i32>().unwrap());
    }

    while pc < program.len() {
        let current_line = program[pc];
        // Part 1: program[pc] += 1;

        if current_line >= 3 {
            program[pc] -= 1;
        } else {
            program[pc] += 1;
        }

        pc = ((pc as i32) + current_line) as usize;
        steps += 1;
    }
    
    println!("Steps: {}", steps);
}
