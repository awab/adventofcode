use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut graph = HashMap::new();
    let mut parents = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut nodes = vec![];
        for index in 2..parts.len() {
            nodes.push(parse_node(parts[index]));
            let entry = parents.entry(parts[index].to_string()).or_insert(vec![]);
            (*entry).push(parts[0].to_string());
        }

        graph.insert(parts[0].to_string(), nodes);
    }

    let mut visited = vec![];
    let mut groups = 1;
    let mut total = 0;
    total += count_nodes(&graph, &mut visited, "0".to_string());
    for key in graph.keys() {
        if visited.contains(&key) {
            continue;
        }

        let node_count = count_nodes(&graph, &mut visited, key.to_string());
        groups += 1;
    }

    //Part 1: count_nodes(&graph, &mut visited, "0".to_string()); 
    println!("Total: {}", groups);
}

fn parse_node(node: &str) -> String {
    if node.ends_with(",") {
        node[0..node.len() - 1].to_string()
    } else {
        node.to_string()
    }
}

fn count_nodes(graph: &HashMap<String, Vec<String>>, visited: &mut Vec<String>, node: String) -> u16 {
    let children = graph.get(&node);
    if children == None {
        visited.push(node);
        return 1;
    }

    visited.push(node.clone());

    let mut total = 1;
    for child in children.unwrap() {
        if visited.contains(&child) {
            continue;
        }

        total += count_nodes(&graph, visited, child.to_string());
    }

    total
}