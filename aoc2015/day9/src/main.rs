use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};
use std::fs::read_to_string;
use std::rc::Rc;

use priority_queue::PriorityQueue;
use regex::Regex;

#[derive(PartialEq, Clone)]
struct Edge {
    pub weight: u32,
    pub destination: Rc<str>,
}

struct Graph {
    nodes: Vec<Rc<str>>,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn has_node(&self, node: Rc<str>) -> bool {
        self.get_node_index(node) != None
    }

    fn add_node(&mut self, node: Rc<str>) -> usize {
        let node_index = self.nodes.len();

        self.nodes.push(node.into());
        self.edges.push(Vec::new());

        node_index
    }

    fn add_edge(&mut self, from_node: Rc<str>, to_node: Rc<str>, weight: u32) {
        let edge = Edge {
            weight: weight,
            destination: to_node
        };

        let index = self.get_node_index(from_node).unwrap();
        self.edges[index].push(edge);
    }

    fn get_node(&self, index: usize) -> &Rc<str> {
        self.nodes.get(index).unwrap()
    }

    fn get_nodes(&self) -> &Vec<Rc<str>> {
        &self.nodes
    }

    fn get_edges(&self, index: usize) -> &Vec<Edge> {
        self.edges.get(index).unwrap()
    }

    fn get_node_index(&self, name: Rc<str>) -> Option<usize> {
        let mut index = 0;
        for node in self.nodes.iter() {
            if *node == name {
                return Some(index);
            }

            index += 1;
        }

        None
    }

    fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    fn get_shortest_path(&self, search_node: Rc<str>) -> Vec<Edge> {
        let mut distances: HashMap<Rc<str>, u32> = HashMap::new();
        let mut visited: Vec<Rc<str>> = vec![];
        for node in self.nodes.iter() {
            if *node == search_node {
                distances.insert(node.clone(), 0);
            } else {
                distances.insert(node.clone(), u32::MAX);
            }
        }

        for _ in 1..=self.get_node_count() {
            // we get the unvisited node with the smallest distance
            let mut shortest_distance = u32::MAX;
            let mut shortest_node = None;
            for (key, value) in distances.clone().into_iter() {
                if visited.iter().any(|i| *i == key) {
                    continue;
                }

                if value < shortest_distance {
                    shortest_distance = value;
                    shortest_node = Some(key);
                }
            }

            if shortest_node != None {
                let current_node = shortest_node.unwrap();
                let current_distance = shortest_distance;
                let mut shortest_distance = u32::MAX;
                let mut shortest_node = None;
                for edge in self.get_edges(self.get_node_index(current_node.clone()).unwrap()).iter() {
                    if visited.iter().any(|i| *i == edge.destination) {
                        continue;
                    }

                    if current_distance + edge.weight < shortest_distance {
                        shortest_distance = current_distance + edge.weight;
                        shortest_node = Some(edge.destination.clone());
                    }
                }    

                if shortest_node != None {
                    distances.insert(shortest_node.unwrap().clone(), shortest_distance);
                }

                visited.push(current_node.clone());
            }
        }        

        // we know the first node
        if distances.keys().len() != self.get_node_count() {
            return vec![];
        }

        // flip the distance key/values; BTreeMap sorts the keys by default
        let reversed_distances: BTreeMap<u32, Rc<str>> = distances.iter().map(|(k,v)| (*v, k.clone())).collect();
        for (key, value) in reversed_distances.clone().into_iter() {
            println!("{} - {}", key, value);
        }
        vec![]
    }


    fn get_longest_path(&self, search_node: Rc<str>) -> Vec<Edge> {
        let mut distances: HashMap<Rc<str>, u32> = HashMap::new();
        let mut visited: Vec<Rc<str>> = vec![];
        for node in self.nodes.iter() {
            if *node == search_node {
                distances.insert(node.clone(), 1000000);    //something absurd
            } else {
                distances.insert(node.clone(), 0);
            }
        }

        for _ in 1..=self.get_node_count() {
            // we get the unvisited node with the smallest distance
            let mut longest_distance = 0;
            let mut longest_node = None;
            for (key, value) in distances.clone().into_iter() {
                if visited.iter().any(|i| *i == key) {
                    continue;
                }

                if value > longest_distance {
                    longest_distance = value;
                    longest_node = Some(key);
                }
            }

            if longest_node != None {
                let current_node = longest_node.unwrap();
                let current_distance = longest_distance;
                let mut longest_distance = 0;
                let mut longest_node = None;
                for edge in self.get_edges(self.get_node_index(current_node.clone()).unwrap()).iter() {
                    if visited.iter().any(|i| *i == edge.destination) {
                        continue;
                    }

                    if current_distance + edge.weight > longest_distance {
                        longest_distance = current_distance + edge.weight;
                        longest_node = Some(edge.destination.clone());
                    }
                }    

                if longest_node != None {
                    distances.insert(longest_node.unwrap().clone(), longest_distance);
                }

                visited.push(current_node.clone());
            }
        }        

        // we know the first node
        if distances.keys().len() != self.get_node_count() {
            return vec![];
        }

        // flip the distance key/values; BTreeMap sorts the keys by default
        let reversed_distances: BTreeMap<u32, Rc<str>> = distances.iter().map(|(k,v)| (*v, k.clone())).collect();
        for (key, value) in reversed_distances.clone().into_iter() {
            println!("{} - {}", key, value);
        }

        vec![]
    }
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"([^\s]+)\sto\s([^\s]+)\s=\s(\d+)").unwrap();
    let mut graph = Graph::new();

    for line in lines {
        for (_, [from_place, to_place, distance]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let from_place = from_place;
            let to_place = to_place;
            let distance_converted = distance.parse::<u32>().unwrap();
            if !graph.has_node(from_place.into()) {
                graph.add_node(from_place.into());    
            }
            if !graph.has_node(to_place.into()) {
                graph.add_node(to_place.into());
            }

            // we add edges in both directions
            graph.add_edge(from_place.into(), to_place.into(), distance_converted);
            graph.add_edge(to_place.into(), from_place.into(), distance_converted);
        }
    }

    let shortest_path_length: u32 = 0xFFFFFFFF; // set to "max"
    for node in graph.get_nodes().iter() {
        println!("Longest path from: {}", node.clone());
        let longest_path_from_node = graph.get_longest_path(Rc::clone(node));
    }
}

#[cfg(test)]
mod test {
    use super::Graph;

    #[test]
    fn test_add_and_retrieve_node() {
        let mut graph = Graph::new();
        let node1 = "test1";
        let node2 = "test2";
        graph.add_node(node1.into());
        graph.add_node(node2.into());
        assert_eq!(graph.get_node_index(node2.into()).unwrap(), 1)
    }

    #[test]
    fn test_add_and_retrieve_node_index() {
        let mut graph = Graph::new();
        let node = "test";
        let index = graph.add_node(node.into());
        assert_eq!(*graph.get_node(index), node.into())
    }

    #[test]
    fn test_add_and_check_edge() {
        let mut graph = Graph::new();
        let node1 = "test";
        let node2 = "test2";
        graph.add_node(node1.into());
        graph.add_node(node2.into());
        graph.add_edge(node1.into(), node2.into(), 10);
        assert_eq!(*g.get(index) == node)
    }
}