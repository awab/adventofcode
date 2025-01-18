use std::collections::HashMap;
use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"(.*)-(\d+)\[([a-z]+)\]").unwrap();

    let mut sector_id_sum: u32 = 0;

    for line in lines {
        for (_, [roomname, sector_id, checksum]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let mut checksum_letters = HashMap::new();
            for ch in checksum.chars() {
                checksum_letters.insert(ch, 0u16);
            }

            for ch in roomname.chars() {
                if checksum_letters.contains_key(&ch) {
                    *checksum_letters.entry(ch).or_insert(0) += 1;
                }
            }
            
            let mut valid = true;
            let mut last_char = '\0';
            let mut running_check = u16::MAX;
            for ch in checksum.chars() {
                // these are in order of the way the counts should be
                let current_value = checksum_letters.get(&ch).unwrap();
                if *current_value == 0 {
                    valid = false;
                    break;
                }

                if running_check > *current_value {
                    running_check = *current_value;
                } else if running_check == *current_value {
                    if last_char < ch {
                        running_check = *current_value;        
                    } else {
                        valid = false;
                        break;
                    }
                } else {
                    valid = false;
                    break;
                }

                last_char = ch;
            }

            if valid {
                // Part 1: sector_id_sum += sector_id.parse::<u32>().unwrap();
                let mut result = String::from("");
                for ch in roomname.chars() {
                    if ch == '-' {
                        result.push(' ');
                    } else {
                        let mut offset_char: u32 = ch as u32 - 97; // 'a' is 97
                        offset_char += sector_id.parse::<u32>().unwrap();
                        offset_char %= 26;
                        offset_char += 97;
                        result.push(char::from_u32(offset_char).unwrap());    
                    }
                }

                println!("{} {}", result, sector_id);
            }
        }
    }
}
