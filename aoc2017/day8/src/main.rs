use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut registers = HashMap::new();
    let mut max = 0;
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let register_check = parts[4].to_string();
        let register_value = *registers.entry(register_check).or_insert(0);
        let check_value = parts[6].parse::<i32>().unwrap();
        let check = match parts[5] {
          ">" => {
            register_value > check_value
          },
          ">=" => {
            register_value >= check_value
          },
          "<" => {
            register_value < check_value
          },
          "<=" => {
            register_value <= check_value
          },
          "==" => {
            register_value == check_value
          },
          "!=" => {
            register_value != check_value
          },
          _ => panic!("Unimplemented operation."),
        };

        if check {
            let adjust_register_value = registers.entry(parts[0].to_string()).or_insert(0);
            let value = parts[2].parse::<i32>().unwrap();
            if parts[1] == "inc" {
                *adjust_register_value += value;
            } else {
                *adjust_register_value -= value;
            }

            if *adjust_register_value > max {
                max = *adjust_register_value;
            }
        }
    }

    /* Part 1
    let mut max = 0;
    for key in registers.keys() {
        let value = *registers.get(key).unwrap();
        if value > max {
            max = value;
        }
    }
    */

    println!("Max value: {}", max);
}
