fn main() {
    let mut input = String::from("01111001100111011");
    let expected_length = 35651584;

    while input.len() <= expected_length {
        input = generate_data(input);
    }

    let mut checksum = input.clone();
    loop {
        checksum = generate_checksum(checksum, expected_length);
        if checksum.len() % 2 != 0 {
            break;
        }
    }

    println!("Valid checksum: {}", checksum);
}

fn generate_data(a: String) -> String {
    let mut b = a.clone();
    b = b.chars().rev().collect::<String>();
    let mut flipped_result = String::from("");
    for ch in b.chars() {
        if ch == '1' {
            flipped_result.push('0');
        } else {
            flipped_result.push('1');
        }
    }

    a + &String::from("0") + &flipped_result
}

fn generate_checksum(input: String, length: usize) -> String {
    let mut data = input;
    if data.len() > length {
        data = data[0..length].to_string();
    }

    let mut ch = data.chars();
    let mut checksum = String::from("");
    loop {
        let char1 = ch.next();
        let char2 = ch.next();

        if char1 == None || char2 == None {
            break;
        }

        if char1.unwrap() == char2.unwrap() {
            checksum.push('1');
        } else {
            checksum.push('0');
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_data_works() {
        // Examples from AOC site
        assert_eq!(generate_data(String::from("1")), String::from("100"));
        assert_eq!(generate_data(String::from("0")), String::from("001"));
        assert_eq!(generate_data(String::from("11111")), String::from("11111000000"));
        assert_eq!(generate_data(String::from("111100001010")), String::from("1111000010100101011110000"));
    }

    #[test]
    fn generate_checksum_works() {
        // Examples from AOC site
        assert_eq!(generate_checksum(String::from("10000011110010000111110"), 20), String::from("0111110101"));
        assert_eq!(generate_checksum(String::from("0111110101"), 20), String::from("01100"));
    }
}
