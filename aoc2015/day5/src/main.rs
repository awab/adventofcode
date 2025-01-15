use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut nice_count = 0;
/* Day 1
    for line in lines {
        let mut vowel_count = 0;
        let mut last_char = '\0';
        let mut has_two_in_a_row = false;
        let mut invalid_pair = false;

        for ch in line.chars() {
            if !has_two_in_a_row && ch == last_char {
                has_two_in_a_row = true;
            } 

            if last_char == 'a' && ch == 'b' ||
            last_char == 'c' && ch == 'd' ||
            last_char == 'p' && ch == 'q' ||
            last_char == 'x' && ch == 'y' {
                // invalid "nice" strings
                invalid_pair = true;
            }

            match ch {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => { },
            };

            last_char = ch;
        }

        if vowel_count >= 3 && has_two_in_a_row && !invalid_pair {
            nice_count += 1;
        }
    }
*/
    
    for line in lines {
        let mut found_pairs: Vec<String> = vec![];
        let mut has_repeating_pair = false;
        let mut has_repeating_with_spacer = false;
        let mut second_to_last_char = '\0';
        let mut last_char = '\0';
        
        for ch in line.chars() {
            if !has_repeating_pair && last_char != '\0' {
                let current_pair = format!("{}{}", last_char, ch);
                if second_to_last_char != ch || second_to_last_char != last_char {
                    // make sure we aren't in an extended repeating string
                    if found_pairs.contains(&current_pair) {
                        has_repeating_pair = true;
                    } else {
                        found_pairs.push(current_pair);
                    }
                }
            }

            if !has_repeating_with_spacer && second_to_last_char != '\0' && last_char != '\0' {
                if second_to_last_char == ch {
                    has_repeating_with_spacer = true;
                }
            }

            second_to_last_char = last_char;
            last_char = ch;
        }

        if has_repeating_pair && has_repeating_with_spacer {
            println!("Nice: {}", line);
            nice_count += 1;
        } else {
            println!("Naughty: {}", line);
        }
    }

    println!("Nice count: {}", nice_count);
}
