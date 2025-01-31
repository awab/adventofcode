fn main() {
    let mut input = String::from(".^^..^...^..^^.^^^.^^^.^^^^^^.^.^^^^.^^.^^^^^^.^...^......^...^^^..^^^.....^^^^^^^^^....^^...^^^^..^");
    let mut results = vec![input.clone()];

    for i in 0..399999 {
        input = get_next_row(input);
        results.push(input.clone());
    }

    let mut safe_count = 0;
    for result in results {
        for ch in result.chars() {
            if ch == '.' {
                safe_count += 1;
            }
        }
    }

    println!("Safe count: {}", safe_count);
}

fn get_next_row(input: String) -> String {
    let mut result = String::from("");
    
    // char 0 is special since it uses an "invisible character"
    let mut check = String::from(".") + &input[0..2];
    if is_trap(check) {
        result.push('^');
    } else {
        result.push('.');
    }

    for index in 1..input.len() - 1 {
        check = input[(index - 1)..=(index + 1)].to_string();
        if is_trap(check) {
            result.push('^');
        } else {
            result.push('.');
        }
    }

    // the last char is special since it uses an "invisible character"
    check = input[(input.len() - 2)..input.len()].to_string() + &String::from(".");
    if is_trap(check) {
        result.push('^');
    } else {
        result.push('.');
    }

    result
}

fn is_trap(input: String) -> bool {
    input == "^^." || input == ".^^" || input == "^.." || input == "..^"
}