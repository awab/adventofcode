use std::collections::HashMap;

use md5;

/* Part 1
fn main() {
    let salt = "ahsbgdzn".to_string();
    let mut prefix: i64 = 0;
    let mut result_count = 0;
    let mut key_searching = false;
    let mut key = String::from("");
    let mut key_index = 0;
    let mut starting_point = 0;

    'md5: loop {
        prefix += 1;
        let digest = md5::compute(salt.to_string() + &prefix.to_string());
        let digest_hex = format!("{:x}", digest);

        if key_searching {
            key_index += 1;
            if digest_hex.contains(&key) {
                result_count += 1;
                key_searching = false;
                prefix = starting_point;
                if result_count == 64 {
                    break 'md5;
                } else {
                    continue 'md5;
                }
            }

            if key_index == 1000 {
                // we made it through 1000 hashes without finding a winner, so we reset
                key_searching = false;
                prefix = starting_point; // reset our check to the starting point
            }
        } else {
            let mut next_to_last_char = '\0';
            let mut last_char = '\0';
    
            for i in digest_hex.chars() {
                if i == last_char && last_char == next_to_last_char {
                    key_searching = true;
                    key_index = 0;
                    starting_point = prefix;
                    key = (0..5).map(|_| i.to_string()).collect::<String>();
                    continue 'md5;
                }

                next_to_last_char = last_char;
                last_char = i;
            }    
        }
    }

    println!("64th key landed on: {}", prefix);
}
*/

fn main() {
    let salt = "ahsbgdzn".to_string();
    let mut prefix: i64 = 0;
    let mut result_count = 0;
    let mut key_searching = false;
    let mut key = String::from("");
    let mut key_index = 0;
    let mut starting_point = 0;
    let mut results: HashMap<i64, String> = HashMap::new();

    'md5: loop {
        prefix += 1;
        let mut digest_hex = String::from("");
        if results.contains_key(&prefix) {
            digest_hex = results.get(&prefix).unwrap().clone();
        } else {
            let mut digest = md5::compute(salt.to_string() + &prefix.to_string());
            for _ in 0..2016 {
                let digest_hex = format!("{:x}", digest);
                digest = md5::compute(digest_hex);
            }
            
            digest_hex = format!("{:x}", digest);
            results.insert(prefix, digest_hex.clone());
        }

        if key_searching {
            key_index += 1;
            if digest_hex.contains(&key) {
                result_count += 1;
                key_searching = false;
                prefix = starting_point;
                if result_count == 64 {
                    break 'md5;
                } else {
                    continue 'md5;
                }
            }

            if key_index == 1000 {
                // we made it through 1000 hashes without finding a winner, so we reset
                key_searching = false;
                prefix = starting_point; // reset our check to the starting point
            }
        } else {
            let mut next_to_last_char = '\0';
            let mut last_char = '\0';
    
            for i in digest_hex.chars() {
                if i == last_char && last_char == next_to_last_char {
                    key_searching = true;
                    key_index = 0;
                    starting_point = prefix;
                    key = (0..5).map(|_| i.to_string()).collect::<String>();
                    continue 'md5;
                }

                next_to_last_char = last_char;
                last_char = i;
            }    
        }
    }

    println!("64th key landed on: {}", prefix);
}