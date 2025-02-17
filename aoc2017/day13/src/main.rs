use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut max_index = 0;
    let mut security_grid = HashMap::new();
    let mut security_index = HashMap::new();
    let mut security_direction_down = HashMap::new();
    let mut keys = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(": ").collect();
        max_index = parts[0].parse::<u32>().unwrap();
        keys.push(max_index);
        security_grid.insert(max_index, parts[1].parse::<u32>().unwrap());
        security_index.insert(max_index, 0);
        security_direction_down.insert(max_index, true);
    }

    let mut severity = 0;
    let mut ticks = 1;

    'check: loop {
        for index in 0..=max_index {
            let depth_check = security_grid.get(&index);
            if depth_check == None {
                continue;
            } else {
                let depth = depth_check.unwrap();
                let position = (ticks + index) % ((depth - 1) * 2);
                if position == 0 {
                    ticks += 1;
                    continue 'check;
                }
            }
        }

        // we found a round that makes it through
        println!("Wait {} ticks", ticks);
        return;
    }

    /* Part 1
    for index in 0..=max_index {
        if security_index.contains_key(&index) {
            // there is a firewall layer
            let value = *security_index.get(&index).unwrap();
            if value == 0 {
                // we were caught
                let depth = security_grid.get(&index).unwrap();
                severity += depth * index;
            }
        }

        // advance the indexes either way
        for key in &keys {
            let value = security_index.get_mut(&key).unwrap();
            let depth_check = security_grid.get(&key);
            let is_down = security_direction_down.get_mut(&key).unwrap();
            let mut depth = 0;
            if depth_check == None {
                continue;
            } else {
                depth = *depth_check.unwrap();
            }

            if *is_down {
                *value += 1;
                if *value == depth {
                    *value = depth - 2;
                    *is_down = false;
                }
            } else {
                if *value == 0 {
                    *value = 1;
                    *is_down = true;
                } else {
                    *value -= 1;
                }
            }
        }
    }

    println!("Total severity of trip: {}", severity);
    */

}

fn tick_indexes(keys: &Vec<u32>, security_grid: &HashMap<u32, u32>, security_index: &mut HashMap<u32, u32>, security_direction_down: &mut HashMap<u32, bool>) {
    for key in keys {
        let value = security_index.get_mut(&key).unwrap();
        let depth_check = security_grid.get(&key);
        let is_down = security_direction_down.get_mut(&key).unwrap();
        let mut depth = 0;
        if depth_check == None {
            continue;
        } else {
            depth = *depth_check.unwrap();
        }

        if *is_down {
            *value += 1;
            if *value == depth {
                *value = depth - 2;
                *is_down = false;
            }
        } else {
            if *value == 0 {
                *value = 1;
                *is_down = true;
            } else {
                *value -= 1;
            }
        }
    }
}

fn reset(keys: &Vec<u32>, security_index: &mut HashMap<u32, u32>, security_direction_down: &mut HashMap<u32, bool>) {
    for key in keys {
        security_index.insert(*key, 0);
        security_direction_down.insert(*key, true);
    }
}
        