use md5;

/* Part 1
fn main() {
    let prefix = "ffykfhsq".to_string();
    let mut answer: i64 = 1;
    let mut result = vec![];
    loop {
        answer += 1;
        let digest = md5::compute(prefix.clone() + &answer.to_string());
        let digest_hex = format!("{:x}", digest);
        if digest_hex.starts_with("00000") {
            result.push(digest_hex.chars().nth(5).unwrap().to_string());
            if result.len() == 8 {
                break;
            }
        }
    }

    println!("Password: {}", result.concat());
}
*/

fn main() {
    let prefix = "ffykfhsq".to_string();
    let mut answer: i64 = 1;
    let mut result = ['\0'; 8];
    'md5: loop {
        answer += 1;
        let digest = md5::compute(prefix.clone() + &answer.to_string());
        let digest_hex = format!("{:x}", digest);
        if digest_hex.starts_with("00000") {
            let index_str = digest_hex.chars().nth(5).unwrap().to_string();
            if index_str == "8" || index_str == "9" || index_str == "a" || index_str == "b" || index_str == "c" || index_str == "d" || index_str == "e" || index_str == "f" {
                continue;
            }
            let index = index_str.parse::<usize>().unwrap();
            if result[index] == '\0' {
                // we only use the first result for each slot
                result[index] = digest_hex.chars().nth(6).unwrap();
            }

            for ch in result {
                if ch == '\0' {
                    continue 'md5;
                }
            }

            break 'md5;
        }
    }

    print!("Password: ");
    for i in 0..8 {
        print!("{}", result[i]);
    }

    println!("");
}