use std::collections::HashMap;
use std::fs::read_to_string;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut two_letter_counts = 0;
    let mut three_letter_counts = 0;

    for line in lines {
        let mut letter_counts = HashMap::new();
        for c in line.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        // so it's not counted twice
        let mut has_two = false;
        let mut has_three = false;
        
        for key in letter_counts.keys() {
            let count = *letter_counts.get(&key).unwrap();
            if !has_two && count == 2 {
                has_two = true;
                two_letter_counts += 1;
            }

            if !has_three && count == 3 {
                has_three = true;
                three_letter_counts += 1;
            }

            if has_two && has_three {
                break;
            }
        }
    }    

    println!("Checksum: {}", two_letter_counts * three_letter_counts);
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut boxes = vec![];
    for line in lines {
        boxes.push(line);
    }

    let line_length = boxes[0].len();

    for index in 0..boxes.len() {
        for subindex in (index + 1)..boxes.len() {
            let mut chars_off = 0;
            let mut common_letters = vec![];
            for char_index in 0..line_length {
                if boxes[index].chars().nth(char_index).unwrap() != boxes[subindex].chars().nth(char_index).unwrap() {
                    chars_off += 1;
                } else {
                    common_letters.push(boxes[index].chars().nth(char_index).unwrap());
                }

                if chars_off > 1 {
                    break;
                }
            }

            if chars_off == 1 {
                let common: String = common_letters.into_iter().collect();
                println!("Winner found, common letters: {}", common);
                return;
            }
        }
    }
}