use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
/* Day 1    
    let mut total_string_length = 0;
    let mut total_display_length = 0;
    for line in lines {
        let string_length = line.len();
        let mut display_length = 0;
        let mut iterator = line.chars().into_iter();

        let mut ch = iterator.next();
        while ch != None {
            match ch {
                Some('\\') => {
                    match iterator.next() {
                        Some('x') => {
                            // grab the next two items and only add 1 to the display
                            iterator.next();
                            iterator.next();
                            display_length += 1;
                        },
                        _ => display_length += 1,
                    }
                },
                _ => display_length += 1,
            };

            ch = iterator.next();
        }

        // remove 2 for the double quotes
        display_length -= 2;

        total_string_length += string_length;
        total_display_length += display_length;
    }

    println!("Result: {}", total_string_length - total_display_length);
*/

    let mut total_string_length = 0;
    let mut total_encoded_length = 0;
    for line in lines {
        let inflated_count = line.len() +
        line.matches("\\").count() +
        line.matches("\"").count() +
        2;  // for the "new double quotes" in the encoded string

        println!("{} {}", line, inflated_count);

        total_encoded_length += inflated_count;
        total_string_length += line.len();
    }

    println!("Result: {}", total_encoded_length - total_string_length);
}
