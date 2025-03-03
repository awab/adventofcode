use std::fs::read_to_string;

struct Node {
    pub index: u16,
    pub children_count: u16,
    pub metadata_entries: u16,
    pub children: Vec<Node>,
    pub metadata: Vec<u16>,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let parts: Vec<_> = lines[0].split_whitespace().collect();
    
    // we get the initial entries for the function
    let child_node_count = parts[0].parse::<u16>().unwrap();
    let metadata_entries = parts[1].parse::<u16>().unwrap();
    let mut parts_index = 0;
    let node = build_node(&parts, 0, &mut parts_index);
    let metadata_total = sum_metadata(&node);
    println!("Total: {}", metadata_total); // Part 1

    let value = get_value(&node);
    println!("Value: {}", value); // Part 2
}

fn build_node(parts: &Vec<&str>, node_index: u16, parts_index: &mut usize) -> Node {
    let child_node_count = parts[*parts_index].parse::<u16>().unwrap();
    *parts_index += 1;
    let metadata_entries = parts[*parts_index].parse::<u16>().unwrap();
    let mut node = Node {
        index: node_index,
        children_count: child_node_count,
        metadata_entries: metadata_entries,
        children: vec![],
        metadata: vec![],
    };

    *parts_index += 1;

    let mut new_node_index = node_index;
    for _ in 0..child_node_count {
        new_node_index += 1;
        node.children.push(build_node(&parts, new_node_index, parts_index));
    }

    for _ in 0..metadata_entries {
        node.metadata.push(parts[*parts_index].parse::<u16>().unwrap());
        *parts_index += 1;
    }

    node
}

fn sum_metadata(node: &Node) -> u16 {
    let mut metadata_sum = 0;

    for child in &node.children {
        metadata_sum += sum_metadata(&child);
    }

    for metadata in &node.metadata {
        metadata_sum += metadata;
    }

    metadata_sum
}

fn get_value(node: &Node) -> u16 {
    let mut value = 0;

    if node.children_count > 0 {
        for index in &node.metadata {
            let offset_index = *index - 1;
            if offset_index < node.children_count {
                value += get_value(&node.children[offset_index as usize]);
            }
        }
    } else {
        for metadata in &node.metadata {
            value += *metadata;
        }
    }

    value
}