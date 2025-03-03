use std::collections::HashMap;
use std::fs::read_to_string;

use priority_queue::PriorityQueue;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut requirements = HashMap::new();
    let mut reverse_requirements = HashMap::new();
    for line in &lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let parents = requirements.entry(parts[7]).or_insert(vec![]);
        parents.push(parts[1]);

        let reverse_parents = reverse_requirements.entry(parts[1]).or_insert(vec![]);
        reverse_parents.push(parts[7]);
    }

    //The tree can have multiple parents...
    let mut all_parents = vec![];
    for key in requirements.keys() {
        let mut parent = *key;
        loop {
            let new_parent = requirements.get(parent);
            if new_parent != None {
                parent = new_parent.unwrap()[0];
            } else {
                break;
            }
        }

        if !all_parents.contains(&parent) {
            all_parents.push(parent);
        }
    }

    let mut result = "".to_string();
    let mut queue = PriorityQueue::new();
    let mut visited = vec![];
    for parent in all_parents {
        queue.push(parent, 255 - (parent.chars().nth(0).unwrap() as u8));
    }
    
    while queue.len() > 0 {
        let queue_item = queue.pop().unwrap();
        let current = queue_item.0;

        let required = requirements.get(current);
        if required != None {
            let required = required.unwrap();
            let mut all_done = true;
            for item in required {
                if !visited.contains(item) {
                    all_done = false;
                    break;
                }
            }
    
            if !all_done {
                // we requeue it with reverse priority, if we're not ready to run it
                queue.push(queue_item.0, 255 - queue_item.1);
                continue;
            }                
        }

        result.push_str(current);
        visited.push(current);

        let dependents = reverse_requirements.get(current);
        if dependents != None {
            let dependents = dependents.unwrap();
            for dependent in dependents {
                // we use ascii value as the queue priority and reverse it (since the priority queue goes from low to high)
                queue.push(dependent, 255 - (dependent.chars().nth(0).unwrap() as u8));
            }    
        } else {
            // we're done!
            break;
        }
    }

    println!("{}", result);
}
*/

const WORKER_COUNT: usize = 5;
const STEP_BASE_TIME: usize = 60;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut requirements = HashMap::new();
    let mut all_steps = vec![];
    for line in &lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let parent = parts[1].to_string();
        let child = parts[7].to_string();
        let parents = requirements.entry(child.clone()).or_insert(vec![]);
        parents.push(parent.clone());
        if !all_steps.contains(&parent) {
            all_steps.push(parent.clone());
        }

        if !all_steps.contains(&child) {
            all_steps.push(child.clone());
        }
    }

    let mut result: Vec<String> = vec![];
    let mut worker_steps: [usize; WORKER_COUNT] = [0; WORKER_COUNT];
    let mut workers: HashMap<usize, String> = HashMap::new();

    let mut steps = 0;
    while all_steps.len() != result.len() {
        let mut valid_steps: Vec<_> = all_steps.iter().filter(|x| !result.contains(x)).collect();
        // remove any that are being worked on now
        for key in workers.keys() {
            let value = workers.get(&key).unwrap();
            if valid_steps.contains(&value) {
                let index = valid_steps.iter().position(|x| *x == value).unwrap();
                valid_steps.remove(index); 
            }
        }

        for index in (0..valid_steps.len()).rev() {
            // we check if the requirements are met
            let step = valid_steps[index];
            let dependencies = requirements.get(step);
            if dependencies == None {
                continue;
            }

            let dependencies = dependencies.unwrap();
            if dependencies.iter().any(|x| !result.contains(x)) {
                valid_steps.remove(index);
            }
        }

        valid_steps.sort();

        for index in 0..WORKER_COUNT {
            if worker_steps[index] == 0 && valid_steps.len() > 0 {
                let next_step = valid_steps[0];
                valid_steps.remove(0);
                workers.insert(index, next_step.to_string());
                let next_step_value = next_step.chars().nth(0).unwrap() as usize;
                let step_value = STEP_BASE_TIME + (next_step_value - 64) - 1;
                worker_steps[index] = steps + step_value; 
            }

            if steps != 0 && worker_steps[index] == steps {
                // it's time to do work
                let value = workers.get(&index).unwrap();
                result.push(value.to_string());
                worker_steps[index] = 0;
            } 
        }

        steps += 1;
    }

    println!("Steps: {}", steps);
}