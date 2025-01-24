use std::fs::read_to_string;
use std::iter::Iterator;

/* Part 1
fn main() {
    let filename = "input.txt";
    let input: String = read_to_string(filename).unwrap();
    
    let mut in_pattern = false;
    let mut in_capture = false;
    let mut characters_to_duplicate = 0u16;
    let mut times_to_duplicate = 0u16;
    let mut current_capture = String::from("");
    let mut result = String::from("");
    let mut ch = input.chars().peekable();

    'olo: loop {
        let current_character = ch.nth(0).unwrap();
        if !in_pattern && !in_capture && current_character == '(' {
            in_pattern = true;
        }

        if in_pattern {
            let mut characters_to_duplicate_str = String::from("");
            let mut current_character = ch.next().unwrap();
            while current_character != 'x' {
                characters_to_duplicate_str.push(current_character);
                current_character = ch.next().unwrap();
            }

            characters_to_duplicate = characters_to_duplicate_str.parse::<u16>().unwrap();
            let mut times_to_duplicate_str = String::from("");
            let mut current_character = ch.next().unwrap();
            while current_character != ')' {
                times_to_duplicate_str.push(current_character);
                current_character = ch.next().unwrap();
            }

            times_to_duplicate = times_to_duplicate_str.parse::<u16>().unwrap();
            current_capture = String::from("");
            in_capture = true;
            in_pattern = false;
        } else if in_capture {
            current_capture.push(current_character);
            if current_capture.len() as u16 == characters_to_duplicate {
                for i in 0..times_to_duplicate {
                    result.push_str(&current_capture);
                    in_capture = false;
                }
            }
        }

        if ch.peek() == None {
            break 'olo;
        }
    }

    println!("Result: {}", result);
    println!("Result len: {}", result.len());
}
*/

fn main() {
    let filename = "input.txt";
    let input: String = read_to_string(filename).unwrap();
    
    let mut in_pattern = false;
    let mut ch = input.chars();
    let current_value = get_next_count(&mut ch);
    println!("Result len: {}", current_value);
}

fn get_next_count<I: Iterator<Item = char>>(ch: &mut I) -> usize where <I as Iterator>::Item: PartialEq<char> {
    let current_character = ch.nth(0);
    if current_character == None || current_character == Some('\n') {
        return 0;
    }

    let current_character = current_character.unwrap();
    let mut characters_to_duplicate: usize = 0;
    let mut times_to_duplicate: usize = 0;

    if current_character == '(' {
        let mut characters_to_duplicate_str = String::from("");
        let mut current_character = ch.next().unwrap();
        while current_character != 'x' {
            characters_to_duplicate_str.push(current_character);
            current_character = ch.next().unwrap();
        }

        characters_to_duplicate = characters_to_duplicate_str.parse::<usize>().unwrap();
        let mut times_to_duplicate_str = String::from("");
        let mut current_character = ch.next().unwrap();
        while current_character != ')' {
            times_to_duplicate_str.push(current_character);
            current_character = ch.next().unwrap();
        }

        times_to_duplicate = times_to_duplicate_str.parse::<usize>().unwrap();
    } else {
        return 1 + get_next_count(ch);
    }

    let str = ch.take(characters_to_duplicate).collect::<String>();
    if str.contains("(") {
        return (times_to_duplicate as usize * get_next_count(&mut str.chars())) + get_next_count(ch);
    } 
    
    (characters_to_duplicate * times_to_duplicate + get_next_count(ch)).into()
}