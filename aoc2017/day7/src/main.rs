
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    /* Part 1:
    let mut parents = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() > 2 {
            // we only care about nodes that contain other nodes
            for i in 3..parts.len() {
                if parts[i].ends_with(",") {
                    parents.insert(parts[i][0..parts[i].len() - 1].to_string(), parts[0].to_string());
                } else {
                    parents.insert(parts[i].to_string(), parts[0].to_string());
                }
            }
        }
    }

    let mut random_key = parents.keys().into_iter().nth(0).unwrap();
    loop {
        if !parents.contains_key(random_key) {
            println!("Parent node: {}", random_key);
            break;
        }

        random_key = parents.get(random_key).unwrap();
    }
    */

    let mut graph = HashMap::new();
    let mut weights = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        weights.insert(parts[0].to_string(), parse_weight(parts[1]));
        if parts.len() > 2 {
            // this node has child nodes
            let mut children = vec![];

            for i in 3..parts.len() {
                if parts[i].ends_with(",") {
                    children.push(parts[i][0..parts[i].len() - 1].to_string());
                } else {
                    children.push(parts[i].to_string());
                }
            }

            graph.insert(parts[0].to_string(), children);
        } else {
            graph.insert(parts[0].to_string(), vec![]);
        }
    }

    let root_node = "vmpywg".to_string();   // answer from Part 1
    check_weights(&graph, &weights, root_node);

    /*
    for key in graph.keys() {
        let children = graph.get(key).unwrap();
        let mut value = 0;
        for child in children {
            let weight = weights.get(child).unwrap();
            if value == 0 {
                value = *weight;
            } else {
                if value != *weight {
                    println!("Invalid child {} should be {}", *weight, value);
                }
            }
        }
    }
    */

    println!("");
}

fn parse_weight(weight: &str) -> u32 {
    weight[1..weight.len() - 1].parse::<u32>().unwrap()
}

fn check_weights(graph: &HashMap<String, Vec<String>>, weights: &HashMap<String, u32>, node: String) -> u32 {
    let mut total_weight = *weights.get(&node).unwrap(); // the root node weight
    let children = graph.get(&node).unwrap();
    let mut results = vec![];
    if children.len() > 0 {
        for child in children {
            results.push(check_weights(&graph, &weights, child.clone()));
        }
    
        let value = results[0];
        total_weight += value;
        for index in 1..results.len() {
            total_weight += results[index];
            if value != results[index] {
                let original_weight = *weights.get(&children[index]).unwrap();
                println!("Node {} should have value {} instead of {}", children[index], original_weight - (results[index] - value), original_weight);
                return total_weight;
            }
        }    
    }

    total_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_weight_works() {
        assert_eq!(parse_weight("(1)"), 1);
        assert_eq!(parse_weight("(123)"), 123);
    }
}