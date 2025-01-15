use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"(turn on|turn off|toggle)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
    let mut grid = [[0u16; 1000]; 1000];
    for line in lines {
        for (_, [command, start_x_str, start_y_str, end_x_str, end_y_str]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let start_x = start_x_str.parse::<i16>().unwrap();
            let start_y = start_y_str.parse::<i16>().unwrap();
            let end_x = end_x_str.parse::<i16>().unwrap();
            let end_y = end_y_str.parse::<i16>().unwrap();

            for y_index in start_y..=end_y {
                for x_index in start_x..=end_x {
                    match command {
                        "turn on" => {
                            grid[x_index as usize][y_index as usize] += 1;
                        },
                        "turn off" => {
                            let mut value = grid[x_index as usize][y_index as usize];
                            if value <= 1 {
                                grid[x_index as usize][y_index as usize] = 0;
                            } else {
                                grid[x_index as usize][y_index as usize] -= 1;        
                            }
                        },
                        "toggle" => {
                            grid[x_index as usize][y_index as usize] += 2;
                        },
                        _ => panic!("Unexpected command."),
                    };
                }
            }
        }
    }

    let mut light_value: u32 = 0;
    for y_check in 0..=999 {
        for x_check in 0..=999 {
            light_value += grid[x_check as usize][y_check as usize] as u32;
        }
    }

    println!("Light value: {}", light_value);
}
