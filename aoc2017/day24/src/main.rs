use std::fs::read_to_string;

use find_all::FindAll;
use priority_queue::PriorityQueue;

#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    pub current_strength: u32,
    pub current_magnet: (u32, u32),
    pub total_strength: u32,
    pub total_length: u32,
    pub visited: Vec<(u32, u32)>,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut magnets = vec![];
    for line in lines {
        let parts: Vec<_> = line.split("/").collect();
        magnets.push((parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap()));
    }

    let initial_state = State {
        current_strength: 0,
        current_magnet: (0, 0),
        total_strength: 0,
        total_length: 0,
        visited: vec![],
    };

    let mut magnets_to_process = PriorityQueue::new();
    magnets_to_process.push(initial_state.clone(), 0);
    // Part 1: let mut strongest_path = initial_state.clone();
    let mut longest_path = initial_state.clone();

    while magnets_to_process.len() > 0 {
        let mut current_state = magnets_to_process.pop().unwrap().0;
        let possible_matches = magnets.iter().find_all(|x| (x.0 == current_state.current_strength || x.1 == current_state.current_strength)).unwrap();

        if possible_matches.len() <= 1 {
            // we'll always find the current magnet, when we hit 1, we're done
            
            /* Part 1
            if current_state.total_strength + current_state.current_magnet.0 + current_state.current_magnet.1 > strongest_path.total_strength {
                // we have to add both sides of the magnet to the total
                current_state.total_strength += current_state.current_magnet.0 + current_state.current_magnet.1;
                strongest_path = current_state.clone();
            }
            */

            if current_state.total_length + 1 > longest_path.total_length
                || ((current_state.total_length + 1 == longest_path.total_length) && current_state.total_strength + current_state.current_magnet.0 + current_state.current_magnet.1 > longest_path.total_strength) {
                current_state.total_length += 1;
                // we have to add both sides of the magnet to the total
                current_state.total_strength += current_state.current_magnet.0 + current_state.current_magnet.1;
                longest_path = current_state.clone();
            }

            continue;
        }

        // we have to add both sides of the magnet to the total
        let total_strength = current_state.total_strength + current_state.current_magnet.0 + current_state.current_magnet.1;
        
        for possible_match in possible_matches {
            // find_all returns indexes
            let possible_match = magnets[possible_match];

            // skip the current magnet - we have to check both ways
            if possible_match.0 == current_state.current_magnet.0 && possible_match.1 == current_state.current_magnet.1 
                || possible_match.0 == current_state.current_magnet.1 && possible_match.1 == current_state.current_magnet.0 {
                continue;
            }

            if current_state.visited.iter().any(|x| x.0 == possible_match.0 && x.1 == possible_match.1) {
                continue;
            }

            let mut new_state = current_state.clone();
            // visit the current magnet
            new_state.visited.push(new_state.current_magnet);
            new_state.current_magnet = possible_match;
            
            new_state.total_length += 1;
                
            // we need to know which side of the magnet to use
            let mut new_current_strength = possible_match.0;
            if new_current_strength == current_state.current_strength {
                new_current_strength = possible_match.1;
            }

            new_state.current_strength = new_current_strength;
            new_state.total_strength = total_strength;

            magnets_to_process.push(new_state.clone(), total_strength);
        }
    }

    // Part 1: println!("Strongest bridge: {}", strongest_path.total_strength);
    println!("Strength of longest path: {}", longest_path.total_strength);
}
