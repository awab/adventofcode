use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut grid = vec![];
    // lines[0] is "initial state: #..#."
    let initial_state = &lines[0][14..lines[0].len()];
    for ch in initial_state.trim().chars() {
        if ch == '#' {
            grid.push(true);
        } else {
            grid.push(false);
        }
    }

    let mut rules: HashMap<(bool, bool, bool, bool, bool), bool> = HashMap::new();
    for index in 2..lines.len() {
        let rule = &lines[index];
        let parts: Vec<_> = rule.split(" => ").collect();
        let rule_input = parts[0].chars().map(|x| if x == '#' { true } else { false } ).collect_tuple().unwrap();
        let rule_output = if parts[1].chars().nth(0).unwrap() == '#' { true } else { false };
        rules.insert(rule_input, rule_output);
    }

    let mut new_grid = vec![];
    let generations = 20;
    let mut zero_index = 0;
    for _ in 0..generations {
        for _ in 0..4 {
            // add four false to the start and end so the first and last items can be handled
            grid.insert(0, false);
            grid.push(false);
        }

        // now we start at the third character [2] with our first real character at [4]
        for index in 2..(grid.len() - 2) {
            let offset = index - 2;
            let end = index + 2;
            let slice: (bool, bool, bool, bool, bool) = grid[offset..=end].iter().map(|x| *x).collect_tuple().unwrap();
            let rule = rules.get(&slice);
            if rule != None {
                let value = *rule.unwrap();
                if value && index <= 3 {
                    // if we insert in the beginning, we offset our 0 marker
                    zero_index += 1;
                }

                new_grid.push(value);
            } else {
                new_grid.push(false);
            }
        }

        grid = new_grid.clone();
        new_grid = vec![];
        
        // we added four characters and converted them to 2, we need to offset our [0] marker
        zero_index += 2;
    }

    let mut index = -zero_index;
    let mut total_potted = 0;
    for item in grid {
        if item {
            total_potted += index;
        }

        index += 1;
    }

    println!("Total potted: {}", total_potted);
}
*/

fn main() {
    // running successive generations (100, 1000, 2000, 5000, 10000) 
    // shows we end up with 25 elements with 5 before our generation count and 20
    // after in the same indexes
    let generations = 50000000000;
    let mut total: u128 = 0;
    total += (generations - 26);
    total += (generations - 22);
    total += (generations - 15);
    total += (generations - 9);
    total += (generations - 5);
    total += (generations + 5);
    total += (generations + 9);
    total += (generations + 15);
    total += (generations + 19);
    total += (generations + 23);
    total += (generations + 28);
    total += (generations + 36);
    total += (generations + 42);
    total += (generations + 48);
    total += (generations + 54);
    total += (generations + 60);
    total += (generations + 64);
    total += (generations + 68);
    total += (generations + 72);
    total += (generations + 76);
    total += (generations + 80);
    total += (generations + 86);
    total += (generations + 90);
    total += (generations + 94);
    total += (generations + 99);

    println!("Total: {}", total);
}