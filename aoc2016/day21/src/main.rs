use std::fs::read_to_string;

/* Part 1
fn main() {
    let mut result = String::from("abcdefgh");
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        match parts[0] {
            "swap" => {
                match parts[1] {
                    "letter" => {
                        result = swap_letter(result, parts[2].chars().nth(0).unwrap(), parts[5].chars().nth(0).unwrap());
                    },
                    "position" => {
                        result = swap_position(result, parts[2].parse::<usize>().unwrap(), parts[5].parse::<usize>().unwrap());
                    },
                    _ => panic!("Unknown swap type!"),
                }
            },
            "rotate" => {
                match parts[1] {
                    "right" => {
                        result = rotate_right(result, parts[2].parse::<usize>().unwrap());
                    },
                    "left" => {
                        result = rotate_left(result, parts[2].parse::<usize>().unwrap());
                    },
                    "based" => {
                        result = rotate_right_position(result, parts[6].chars().nth(0).unwrap());
                    },
                    _ => panic!("Unknown rotation type!"),
                }
            },
            "reverse" => {
                result = reverse(result, parts[2].parse::<usize>().unwrap(), parts[4].parse::<usize>().unwrap());
            },
            "move" => {
                result = move_position(result, parts[2].parse::<usize>().unwrap(), parts[5].parse::<usize>().unwrap());
            },
            _ => panic!("Unknown command!"),
        };
    }

    println!("Result: {}", result);
}
*/

fn main() {
    let mut result = String::from("fbgdceah");
    
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().rev().map(String::from).collect();
    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        match parts[0] {
            "swap" => {
                match parts[1] {
                    "letter" => {
                        result = swap_letter(result, parts[2].chars().nth(0).unwrap(), parts[5].chars().nth(0).unwrap());
                    },
                    "position" => {
                        result = swap_position(result, parts[2].parse::<usize>().unwrap(), parts[5].parse::<usize>().unwrap());
                    },
                    _ => panic!("Unknown swap type!"),
                }
            },
            "rotate" => {
                match parts[1] {
                    "right" => {
                        result = rotate_left(result, parts[2].parse::<usize>().unwrap());
                    },
                    "left" => {
                        result = rotate_right(result, parts[2].parse::<usize>().unwrap());
                    },
                    "based" => {
                        let ch = parts[6].chars().nth(0).unwrap();
                        let original_index = result.chars().position(|x| ch == x).unwrap();
                        // figured out with manual math
                        // due to the logic, each index shifts to a unique position
                        let shift_value = if original_index == 0 {
                            1
                        } else if original_index == 1 {
                            1
                        } else if original_index == 2 {
                            6
                        } else if original_index == 3 {
                            2
                        } else if original_index == 4 {
                            7
                        } else if original_index == 5 {
                            3
                        } else if original_index == 6 {
                            0
                        } else if original_index == 7 {
                            4
                        } else {
                            0
                        };

                        result = rotate_left(result.clone(), shift_value);
                    },
                    _ => panic!("Unknown rotation type!"),
                }
            },
            "reverse" => {
                result = reverse(result, parts[2].parse::<usize>().unwrap(), parts[4].parse::<usize>().unwrap());
            },
            "move" => {
                result = move_position(result, parts[5].parse::<usize>().unwrap(), parts[2].parse::<usize>().unwrap());
            },
            _ => panic!("Unknown command!"),
        };
    }

    println!("Result: {}", result);
}

fn swap_position(input: String, x: usize, y: usize) -> String {
    let mut byte_str = input.into_bytes();
    let temp = byte_str[x];
    byte_str[x] = byte_str[y];
    byte_str[y] = temp;
    String::from_utf8(byte_str).unwrap()
}

fn swap_letter(input: String, x: char, y: char) -> String {
    let first_index = input.chars().position(|ch| ch == x).unwrap();
    let second_index = input.chars().position(|ch| ch == y).unwrap();
    
    swap_position(input, first_index, second_index)
}

fn rotate_left(input: String, amount: usize) -> String {
    let amount = amount % input.len();
    let mut result = String::from(&input[amount..]);
    result.push_str(&input[0..amount]);
    
    result
}

fn rotate_right(input: String, amount: usize) -> String {
    let amount = amount % input.len();
    let diff = input.len() - amount;
    let mut result = String::from(&input[diff..]);
    result.push_str(&input[0..diff]);
    
    result
}

// rotate once, rotate index times, rotate again if index >= 4
fn rotate_right_position(input: String, x: char) -> String {
    let index = input.chars().position(|ch| ch == x).unwrap();
    let mut result = rotate_right(input, 1);
    result = rotate_right(result, index);
    if index >= 4 {
        result = rotate_right(result, 1);
    }

    result
}

fn reverse(input: String, first_index: usize, second_index: usize) -> String {
    let mut result = String::from(&input[0..first_index]);
    result += &String::from_iter(&input[first_index..=second_index].chars().rev().collect::<Vec<_>>());
    result.push_str(&input[(second_index + 1)..]);

    result
}

fn move_position(input: String, first_index: usize, second_index: usize) -> String {
    let ch = input.chars().nth(first_index).unwrap();
    
    let input_stripped = input.replace(ch, "");
    let mut result = String::from(&input_stripped[0..second_index]);  
    result.push(ch);
    if second_index < input_stripped.len() {
        result.push_str(&input_stripped[second_index..]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_swap_position() {
        let mut input = String::from("abcdefg");
        assert_eq!(swap_position(input, 0, 1), "bacdefg".to_string());

        input = String::from("abcdefg");
        assert_eq!(swap_position(input, 2, 3), "abdcefg".to_string());

        input = String::from("abcdefg");
        assert_eq!(swap_position(input, 5, 6), "abcdegf".to_string());
    }

    #[test]
    fn check_swap_letter() {
        let mut input = String::from("abcdefg");
        assert_eq!(swap_letter(input, 'a', 'g'), "gbcdefa".to_string());

        input = String::from("abcdefg");
        assert_eq!(swap_letter(input, 'd', 'c'), "abdcefg".to_string());
    }

    #[test]
    fn check_rotate_left() {
        let mut input = String::from("abcdefg");
        assert_eq!(rotate_left(input, 1), "bcdefga".to_string());

        input = String::from("abcdefg");
        assert_eq!(rotate_left(input, 7), "abcdefg".to_string());

        input = String::from("abcdefg");
        assert_eq!(rotate_left(input, 9), "cdefgab".to_string());
    }

    #[test]
    fn check_rotate_right() {
        let mut input = String::from("abcdefg");
        assert_eq!(rotate_right(input, 1), "gabcdef".to_string());

        input = String::from("abcdefg");
        assert_eq!(rotate_right(input, 7), "abcdefg".to_string());

        input = String::from("abcdefg");
        assert_eq!(rotate_right(input, 9), "fgabcde".to_string());
    }

    #[test]
    fn check_rotate_right_position() {
        let mut input = String::from("abdec");
        assert_eq!(rotate_right_position(input, 'b'), "ecabd".to_string());

        input = String::from("ecabd");
        assert_eq!(rotate_right_position(input, 'd'), "decab".to_string());
    }

    #[test]
    fn check_reverse() {
        let mut input = String::from("abcdefg");
        assert_eq!(reverse(input, 2, 4), "abedcfg".to_string());

        input = String::from("abcdefg");
        assert_eq!(reverse(input, 1, 6), "agfedcb".to_string());

        input = String::from("abcdefg");
        assert_eq!(reverse(input, 0, 6), "gfedcba".to_string());
    }

    #[test]
    fn check_move() {
        let mut input = String::from("bcdea");
        assert_eq!(move_position(input, 1, 4), "bdeac".to_string());

        input = String::from("bdeac");
        assert_eq!(move_position(input, 3, 0), "abdec".to_string());
    }
}