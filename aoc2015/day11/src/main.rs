use std::collections::HashMap;

fn main() {
    let mut current_password = String::from("hxbxwxba");
    let mut is_valid_password = is_valid(current_password.clone());
    while !is_valid_password {
        current_password = increment_password(current_password);
        is_valid_password = is_valid(current_password.clone());
    }

    println!("Next valid password: {}", current_password);

    is_valid_password = false;
    while !is_valid_password {
        current_password = increment_password(current_password);
        is_valid_password = is_valid(current_password.clone());
    }

    println!("Next valid password: {}", current_password);
}

fn increment_password(input: String) -> String {
    // it's easier to work backwards
    let reverse = input.chars().rev().collect::<String>();
    let mut carry = false;
    let mut result = String::from("");
    let mut done = false;
    for ch in reverse.chars() {
        let mut new_char = ch;
        if !done {
            if ch == 'z' {
                new_char = 'a';
                carry = true;
            } else {
                new_char = char::from_u32((ch.to_ascii_lowercase() as u32) + 1).unwrap();
                carry = false;
            }

            done = !carry;
        }

        result = vec![result, new_char.to_string()].concat();
    }

    result.chars().rev().collect::<String>()
}

fn is_valid(input: String) -> bool {
    let mut pairs = vec![];
    let mut has_straight = false;

    let mut second_to_last_char = '\0';
    let mut last_char = '\0';
    let mut last_was_pair = false;  // to make sure a triple (aaa) isn't counted as two pairs

    for ch in input.chars() {
        if ch == 'i' || ch == 'o' || ch == 'l' {
            return false;
        }

        if !has_straight {
            let second_to_last_as_int = second_to_last_char as u8;
            let last_as_int = last_char as u8;
            let current_as_int = ch as u8;
            if (last_as_int == second_to_last_as_int + 1) && (current_as_int == last_as_int + 1) {
                has_straight = true;
            }    
        }

        if !last_was_pair {
            if last_char == ch {
                pairs.push(vec![last_char.to_string(), ch.to_string()].concat());
                last_was_pair = true;
            }
        } else {
            last_was_pair = false;
        }

        second_to_last_char = last_char;
        last_char = ch;
    }

    let mut pair_counts: HashMap<String, u16> = HashMap::new();
    for index in pairs {
        *pair_counts.entry(index).or_default() += 1;
    }
    
    has_straight && pair_counts.len() >= 2
}
