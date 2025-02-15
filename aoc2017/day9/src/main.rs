use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let line: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    let mut is_garbage = false;
    let mut is_escaped = false;
    let mut groups = 0;
    let mut total = 0;
    let mut garbage_count = 0;

    for ch in line[0].chars() {
        if !is_escaped && ch == '!' {
            is_escaped = true;
            continue;   // to skip the reset below
        } else if !is_escaped && !is_garbage && ch == '<' {
            is_garbage = true;
        } else if !is_escaped && is_garbage && ch == '>' {
            is_garbage = false;
        } else if !is_escaped && !is_garbage && ch == '{' {
            groups += 1;
        } else if !is_escaped && !is_garbage && ch == '}' {
            total += groups;
            groups -= 1;
        } else if !is_escaped && is_garbage {
            garbage_count += 1;
        }
        
        if is_escaped {
            is_escaped = false;
        }
    }

    println!("Total: {}", total);   // Part 1
    println!("Garbage: {}", garbage_count);   // Part 2
}
