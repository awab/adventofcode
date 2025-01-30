use std::fs::read_to_string;

use regex::Regex;

struct Disc {
    position_count: i8,
    position_index: i8,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"Disc #(\d) has (\d+) positions; at time=0, it is at position (\d+)").unwrap();
    let mut discs = vec![];

    for line in lines {
        for (_, [_, position_count, position_index]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let disc = Disc {
                position_count: position_count.parse::<i8>().unwrap(),
                position_index: position_index.parse::<i8>().unwrap(),
            };

            discs.push(disc);
        }
    }

    // Part 2
    let disc = Disc {
        position_count: 11,
        position_index: 0,
    };
    discs.push(disc);

    let mut time = 0;
    'olo: loop {
        time += 1;
        
        // rotate each disc
        for disc in &mut discs {
            disc.position_index += 1;
            if disc.position_index == disc.position_count {
                disc.position_index = 0;
            }
        }
        
        // check whether the discs are in the right spot
        let mut expected_index: i8 = 0;
        for index in 0..=discs.len() - 1 {
            expected_index -= 1;
            
            let mut expected_position = discs[index].position_count + expected_index;
            if expected_position < 0 {
                expected_position = discs[index].position_count + expected_position;
            }
            if discs[index].position_index != expected_position {
                continue 'olo;
            }
        }

        // if we make it here, all discs should be lined up properly
        println!("Ball should drop at {}", time);
        return;
    }
}
