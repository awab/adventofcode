use std::collections::HashMap;
use std::fs::read_to_string;

use regex::Regex;

#[derive(PartialEq, Clone)]
enum Command {
    AND,
    LSHIFT,
    NOT,
    OR,
    RSHIFT,
    STORE
}

#[derive(Clone)]
struct Node {
    pub command: Command,
    pub left_operand: String,
    pub right_operand: String,
    pub name: String,
}

struct Graph {
    pub nodes: HashMap<String, Node>,
    pub values: HashMap<String, i32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            values: HashMap::new(), 
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.name.clone(), node);
    }
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"(.*?)\s->\s(\w+)").unwrap();
    let mut graph = Graph::new();

    for line in lines {
        for (_, [command, wire]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let command_parts: Vec<&str> = command.split(' ').collect();
            let mut action = Command::STORE;

            if command_parts.len() != 1 {
                action = match command_parts[1] {
                    "AND" => Command::AND,
                    "LSHIFT" => Command::LSHIFT,
                    "OR" => Command::OR,
                    "RSHIFT" => Command::RSHIFT,
                    _ => Command::NOT,
                };
            }

            // if we defaulted, let's see if it's actually a "NOT" command
            if command_parts[0] == "NOT" {
                action = Command::NOT;
            }   
     
            let mut value = -1;
            let mut left_operand = "".to_string();
            if action == Command::NOT {
                left_operand = command_parts[1].to_string();
            } else {
                left_operand = command_parts[0].to_string();
            }

            let mut right_operand = "".to_string();
            if action == Command::NOT {
                right_operand = command_parts[1].to_string();
            } else if action != Command::STORE {
                right_operand = command_parts[2].to_string();
            }

            let node = Node {
                command: action,
                left_operand: left_operand,
                right_operand: right_operand,
                name: wire.to_string(),
            };
            graph.add_node(node);
        }
    }

    // add a hardcoded node setting b
    let b_node = Node {
        command: Command::STORE,
        left_operand: "16076".to_string(),
        right_operand: "".to_string(),
        name: "b".to_string(),
    };
    graph.add_node(b_node);

    let a_node = graph.nodes.get("a").unwrap();
    let result = evaluate(&graph, &mut HashMap::new(), a_node.clone());
    println!("a: {}", result);
}

fn is_numeric(str: String) -> bool {
    let mut all_numeric = true;
    for ch in str.chars() {
        if !ch.is_numeric() {
            all_numeric = false;
            break;
        }
    }

    all_numeric
}

fn evaluate(graph: &Graph, mut values: &mut HashMap<String, i32>, node: Node) -> i32 {
    let mut result = -1;
    let maybe_value = values.get(&node.name);
    if maybe_value != None {
        result = *maybe_value.unwrap();
    }
    
    if result != -1 {
        return result;
    } else if node.command == Command::STORE {
        let mut lvalue = 0;
        if is_numeric(node.left_operand.clone()) {
            lvalue = node.left_operand.parse::<i32>().unwrap();
        } else {
            let lnode = graph.nodes.get(&node.left_operand).unwrap();
            lvalue = evaluate(graph, &mut values, lnode.clone());    
        }

        result = lvalue;        
    } else if node.command == Command::NOT {
        let rnode = graph.nodes.get(&node.right_operand).unwrap();
        let rvalue = evaluate(graph, &mut values, rnode.clone());

        result = !rvalue;
    } else {
        let mut lvalue = 0;
        if is_numeric(node.left_operand.clone()) {
            lvalue = node.left_operand.parse::<i32>().unwrap();
        } else {
            let lnode = graph.nodes.get(&node.left_operand).unwrap();
            lvalue = evaluate(graph, &mut values, lnode.clone());    
        }
        let mut rvalue = 0;
        if is_numeric(node.right_operand.clone()) {
            rvalue = node.right_operand.parse::<i32>().unwrap();
        } else {
            let rnode = graph.nodes.get(&node.right_operand).unwrap();
            rvalue = evaluate(graph, &mut values, rnode.clone());    
        }

        match node.command {
            Command::AND => result = lvalue & rvalue,
            Command::LSHIFT => result = lvalue << rvalue,
            Command::OR => result = lvalue | rvalue,
            Command::RSHIFT => result = lvalue >> rvalue,
            _ => panic!("Unknown command."),
        };
    }

    values.insert(node.name.clone(), result);
    result
}