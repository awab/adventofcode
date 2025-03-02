use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let mut original_line: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    // there is only a single line
    let line = &mut original_line[0].clone();
    let mut index = 0;
    let mut removals = HashMap::new();
    while index < line.len() - 1 {
        let current_char = line.chars().nth(index).unwrap();
        let next_char = line.chars().nth(index + 1).unwrap();

        if current_char != next_char && (current_char == next_char.to_uppercase().next().unwrap() || current_char.to_uppercase().next().unwrap() == next_char) {
            // match, let's remove them in reverse order so the indexes don't change
            line.remove(index + 1);
            line.remove(index);

            let lowercase_ensured_char = current_char.to_lowercase().next().unwrap();
            let count = removals.entry(lowercase_ensured_char).or_insert(0);
            *count += 1;

            if index > 0 {
                index -= 1;
                // so we recheck the last character against the new character                    
            }
        } else {
            index += 1;
        }
    }

    let mut smallest = usize::MAX;

    for key in removals.keys() {
        // simply redo it, after removing the current character
        let mut line = original_line[0].clone().replace(*key, "").replace(key.to_uppercase().next().unwrap(), "");
        let mut index = 0;
        while index < line.len() - 1 {
            let current_char = line.chars().nth(index).unwrap();
            let next_char = line.chars().nth(index + 1).unwrap();

            if current_char != next_char && (current_char == next_char.to_uppercase().next().unwrap() || current_char.to_uppercase().next().unwrap() == next_char) {
                // match, let's remove them in reverse order so the indexes don't change
                line.remove(index + 1);
                line.remove(index);

                if index > 0 {
                    index -= 1;
                    // so we recheck the last character against the new character                    
                }
            } else {
                index += 1;
            }
        }

        if line.len() < smallest {
            smallest = line.len();
        }
    }     

    println!("Remaining units: {}", smallest);
    // Part 1: println!("Remaining units: {}", line.len());
}
