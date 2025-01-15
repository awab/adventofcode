use std::fs::read_to_string;

use itertools::Itertools;

/* Day1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut all_containers = vec![];
    for line in lines {
        all_containers.push(line.parse::<u8>().unwrap());
    }

    let mut total_available_matches = 0;
    for size in 2..all_containers.len() {
        for combination in all_containers.clone().into_iter().combinations(size) {
            let mut current_total: u16 = 0;
            for value in combination {
                current_total += value as u16;
            }

            if current_total == 150 {
                total_available_matches += 1;
            }
        }
    }

    println!("Total available combinations: {}", total_available_matches);
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut all_containers = vec![];
    for line in lines {
        all_containers.push(line.parse::<u8>().unwrap());
    }

    let mut total_available_matches = 0;
    for combination in all_containers.clone().into_iter().combinations(4) {
        let mut current_total: u16 = 0;
        for value in combination {
            current_total += value as u16;
        }

        if current_total == 150 {
            total_available_matches += 1;
        }
    }

    println!("Total available combinations: {}", total_available_matches);
}