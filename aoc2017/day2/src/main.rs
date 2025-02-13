use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    /* Part 1
    let mut checksum = 0;
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut max = 0;
        let mut min = u16::MAX;
        for part in parts {
            let num = part.parse::<u16>().unwrap();
            if num > max {
                max = num;
            }

            if num < min {
                min = num;
            }
        }

        checksum += max - min;
    }

    println!("Checksum: {}", checksum);
    */

    let mut checksum = 0;
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        for combination in parts.into_iter().combinations(2) {
            let num1 = combination[0].parse::<u16>().unwrap();
            let num2 = combination[1].parse::<u16>().unwrap();
            if num1 > num2 {
                if num1 % num2 == 0 {
                    checksum += num1 / num2;
                    break;
                }
            } else {
                if num2 % num1 == 0 {
                    checksum += num2 / num1;
                    break;
                }
            }
        }
    }

    println!("Checksum: {}", checksum);
}
