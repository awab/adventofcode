use std::fs::read_to_string;

/* Day1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut password: Vec<i8> = vec![];
    
    let mut number: i8 = 5;
    for line in lines {
        println!("{}", line);
        for instruction in line.chars() {
            match instruction {
                'U' => {
                    number -= 3;
                    if number <= 0 {
                        number += 3;
                    }
                },
                'D' => {
                    number += 3;
                    if number > 9 {
                        number -= 3;
                    }
                },
                'L' => {
                    let mut minimum = 1;
                    if number >= 7 {
                        minimum = 7;
                    } else if number >= 4 {
                        minimum = 4;
                    }

                    number -= 1;
                    if number < minimum {
                        number = minimum;
                    }
                },
                'R' => {
                    let mut maximum = 9;
                    if number <= 3 {
                        maximum = 3;
                    } else if number <= 6 {
                        maximum = 6;
                    }

                    number += 1;
                    if number > maximum {
                        number = maximum;
                    }
                },
                _ => panic!("Unhandled instruction: {}", instruction),
            }
        }

        password.push(number);
    }

    print!("Password is: ");
    for digit in password {
        print!("{}", digit);
    }
    println!("");
}
*/
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut password: Vec<i8> = vec![];
    
    let mut number: i8 = 5;
    for line in lines {
        for instruction in line.chars() {
            match instruction {
                'U' => {
                    if number == 3 || number == 13 {
                        number -= 2;
                    } else if number != 5 && number != 9 {
                        number -= 4;
                        if number <= 0 {
                            number += 4;
                        }  
                    }
                    // other numbers stay in place
                },
                'D' => {
                    if number == 1 || number == 11 {
                        number += 2;
                    } else if number != 5 && number != 9 {
                        number += 4;
                        if number > 13 {
                            number -= 4;
                        }  
                    }
                    // other numbers stay in place
                },
                'L' => {
                    let mut minimum = 1;
                    if number == 13 {
                        minimum = 13;
                    } else if number >= 10 {
                        minimum = 10;
                    } else if number >= 5 {
                        minimum = 5;
                    } else if number >= 2 {
                        minimum = 2;
                    }

                    number -= 1;
                    if number < minimum {
                        number = minimum;
                    }
                },
                'R' => {
                    let mut maximum = 13;
                    if number == 1 {
                        maximum = 1;
                    } else if number <= 4 {
                        maximum = 4;
                    } else if number <= 9 {
                        maximum = 9;
                    } else if number <= 12 {
                        maximum = 12;
                    }

                    number += 1;
                    if number > maximum {
                        number = maximum;
                    }
                },
                _ => panic!("Unhandled instruction: {}", instruction),
            }
        }

        password.push(number);
    }

    print!("Password is: ");
    for digit in password {
        match digit {
            10 => print!("A"),
            11 => print!("B"),
            12 => print!("C"),
            13 => print!("D"),
            _ => print!("{}", digit),
        };
    }
    println!("");
}