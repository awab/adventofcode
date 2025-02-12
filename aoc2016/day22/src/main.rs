use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;
use priority_queue::PriorityQueue;
use regex::Regex;

const GRID_WIDTH: u8 = 36;
const GRID_HEIGHT: u8 = 26;

#[derive(Eq, PartialEq, Clone, Hash)]
struct Node {
    pub x: u8,
    pub y: u8,
    pub size: u16,
    pub used: u16,
    pub available: u16,
    pub use_percent: u8,
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct State {
    pub zero_node: (u8, u8),
    pub goal_node: (u8, u8),
    pub current_node: (u8, u8),
    pub nodes: Vec<Node>,
    pub move_number: u32,
    pub goal_visited: Vec<(u8, u8)>, // tracks places the goal has been
    pub node_visited: Vec<(u8, u8)>, // tracks places we've checked for space, so we don't end up in infinite loops going back and forth
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    let mut nodes = vec![];
    // the first line is the command and the second is the header, so we skip them...
    for line in &lines[2..] {
        for (_, [x, y, size, used, available, use_percent]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let node = Node {
                x: x.parse::<u8>().unwrap(),
                y: y.parse::<u8>().unwrap(),
                size: size.parse::<u16>().unwrap(),
                used: used.parse::<u16>().unwrap(),
                available: available.parse::<u16>().unwrap(),
                use_percent: use_percent.parse::<u8>().unwrap(),
            };

            nodes.push(node);
        }
    }

    let mut steps: usize = 0;
    let goal_node = nodes.iter().max_by_key(|node| if node.y == 0 { node.x } else { 0u8 }).unwrap();
    let mut goal = (goal_node.x, goal_node.y);
    let mut size_0_node = nodes.iter().find(|&node| node.used == 0).unwrap();
    let mut empty = (size_0_node.x, size_0_node.y);

    let initial_result_empty_to_goal = get_shortest_path(&nodes, (empty.0, empty.1), (goal.0, 0), None);
    steps += initial_result_empty_to_goal.len() - 1;    // -1 for the start
    empty = goal;
    goal.0 -= 1;

    // each x movement takes 5 moves (free space on right, down, over to the left, up)
    steps += (GRID_WIDTH as usize - 1) * 5;

    println!("Steps: {}", steps);
    //println!("Result: {}", result_goal_to_empty + result_empty_to_origin + 1);

    /* Part 2 try 1
    let goal = nodes.iter().max_by_key(|node| if node.y == 0 { node.x } else { 0u8 }).unwrap();
    let mut states: Vec<(Vec<Node>, (u8, u8))> = vec![];

    /* Part 1 below 
    let mut results = vec![];
    for combination in nodes.iter().combinations(2) {
        if combination[0].size > used {
            results.push((combination[0].x, combination[0].y));
        }

        if combination[1].size > used {
            results.push((combination[1].x, combination[1].y));
        }
    }
    */

    let size_0_node = nodes.iter().find(|&node| node.used == 0).unwrap();
    let mut pq = PriorityQueue::new();
    let first_state = State {
        zero_node: (size_0_node.x, size_0_node.y), 
        goal_node: (goal.x, goal.y),
        current_node: (goal.x, goal.y),
        nodes: nodes.clone(),
        move_number: 0, // 0 because we haven't made any moves
        goal_visited: vec![],
        node_visited: vec![],
    };

    pq.push(first_state.clone(), Reverse(0));

    while pq.len() > 0 {
        let mut state = pq.pop().unwrap().0;
        let current_x = state.current_node.0;
        let current_y = state.current_node.1;
        let mut is_goal = false;
        let current_node = state.nodes.iter().find(|&node| node.x == current_x && node.y == current_y).unwrap();
        let current_used = current_node.used;
        
        if states.iter().find(|&nodes| *nodes.0 == state.nodes && nodes.1 == state.current_node) != None {
            //println!("Duplicate: {}, {}", neighbor.x, neighbor.y);
            continue;
        }

        states.push((state.nodes.clone(), state.current_node.clone()));

        if state.current_node == state.goal_node {
            is_goal = true;
        }         

        state.node_visited.push((current_x, current_y)); 
            
        let neighbors = get_neighbors(&state.nodes, current_x, current_y);
        if current_used > 0 {
            for neighbor in &neighbors {
                if neighbor.available >= current_used {
                    if is_goal {
                        // let's make sure we haven't been here before
                        if state.goal_visited.iter().find(|&node| node.0 == neighbor.x && node.1 == neighbor.y) != None {
                            //println!("Duplicate: {}, {}", neighbor.x, neighbor.y);
                            continue;
                        }
 
                        // we should never move backwards
                        if neighbor.x > state.goal_node.0 && neighbor.y > state.goal_node.1 {
                            continue;
                        }

                        // if we move the goal, we update it
                        state.goal_visited.push((current_x, current_y)); 
                    }

                    // we found a node that can take our data
                    let mut new_nodes = state.nodes.clone();
                    move_data(&mut new_nodes, current_x, current_y, neighbor.x, neighbor.y);
                    if is_goal {
                        //println!("Goal Move {}: {},{} to {},{}", state.move_number + 1, current_x, current_y, neighbor.x, neighbor.y);

                        if neighbor.x == 0 && neighbor.y == 0 {
                            // we've successfully moved the data here
                            // it's a priority queue, so this should be the lowest number
                            println!("Took {} moves", state.move_number + 1);
                            return;
                        }      
                    } else {
                        //println!("Move {}: {},{} to {},{}", state.move_number + 1, current_x, current_y, neighbor.x, neighbor.y);
                    }
            
                    let size_0_node = new_nodes.iter().find(|&node| node.used == 0).unwrap();
            
                    let mut new_state = State {
                        zero_node: (size_0_node.x, size_0_node.y),
                        goal_node: state.goal_node.clone(),
                        current_node: state.goal_node.clone(),
                        nodes: new_nodes,
                        move_number: state.move_number + 1, // increment because we moved data
                        goal_visited: state.goal_visited.clone(),
                        node_visited: state.node_visited.clone(),
                    };
    
                    if is_goal {
                        new_state.goal_node = (neighbor.x, neighbor.y);
                        new_state.current_node = (neighbor.x, neighbor.y);
                    }

                    pq.push(new_state.clone(), Reverse(get_ranking(new_state.zero_node, new_state.goal_node)));
                }
            }
        }

        if is_goal {
            // let's try looping over the neighbors
            // of the node that has size == 0
            let size_0_node = state.nodes.iter().find(|&node| node.used == 0).unwrap();
            let neighbors = get_neighbors(&state.nodes, size_0_node.x, size_0_node.y);
            for neighbor in &neighbors {
                if neighbor.available <= current_used {
                    if state.node_visited.iter().find(|&node| node.0 == neighbor.x && node.1 == neighbor.y) != None {
                        //println!("Duplicate: {}, {}", neighbor.x, neighbor.y);
                        continue;
                    }

                    let new_state = State {
                        zero_node: (size_0_node.x, size_0_node.y),
                        goal_node: state.goal_node.clone(),
                        current_node: (neighbor.x, neighbor.y),
                        nodes: state.nodes.clone(),
                        move_number: state.move_number,
                        goal_visited: state.goal_visited.clone(),
                        node_visited: state.node_visited.clone(),
                    };
        
                    pq.push(new_state.clone(), Reverse(get_ranking(new_state.zero_node, new_state.goal_node))); 
                }
            }
        }
    }
    */
}

fn get_neighbors(nodes: &Vec<Node>, x: u8, y: u8) -> Vec<Node> {
    let mut results = vec![];
    
    // up
    if y > 0 {
        let up_entry = nodes.iter().find(|&node| node.x == x && node.y == y - 1);
        if up_entry != None {
            let up_entry = up_entry.unwrap();
            results.push((*up_entry).clone());
        }
    }

    // right
    if x == 0 || x % GRID_WIDTH != 0 {
        let right_entry = nodes.iter().find(|&node| node.x == x + 1 && node.y == y);
        if right_entry != None {
            let right_entry = right_entry.unwrap();
            results.push((*right_entry).clone());
        }
    }

    // down
    if y < GRID_HEIGHT {
        let down_entry = nodes.iter().find(|&node| node.x == x && node.y == y + 1);
        if down_entry != None {
            let down_entry = down_entry.unwrap();
            results.push((*down_entry).clone());
        }
    }

    // left
    if x > 0 {
        let left_entry = nodes.iter().find(|&node| node.x == x - 1 && node.y == y);
        if left_entry != None {
            let left_entry = left_entry.unwrap();
            results.push((*left_entry).clone());
        }
    }
    
    results
}

fn move_data(nodes: &mut Vec<Node>, origin_x: u8, origin_y: u8, target_x: u8, target_y: u8) {
    let origin: &mut Node = nodes.iter_mut().find(|node| node.x == origin_x && node.y == origin_y).unwrap();
    let used = origin.used;
    origin.used = 0;
    origin.available = origin.size;
    let target: &mut Node = nodes.iter_mut().find(|node| node.x == target_x && node.y == target_y).unwrap();
    target.used += used;
    target.available = target.size - target.used;
}

fn get_node_distance(origin: &Node, destination: &Node) -> u8 {
    (origin.x as i8 - destination.x as i8).abs() as u8 + (origin.y as i8 - destination.y as i8).abs() as u8
}

fn get_tuple_distance(origin: (u8, u8), destination: (u8, u8)) -> u8 {
    (origin.0 as i8 - destination.0 as i8).abs() as u8 + (origin.1 as i8 - destination.1 as i8).abs() as u8
}

fn get_ranking(node: (u8, u8), goal: (u8, u8)) -> u8 {
    // A*-style search with a heuristic of the distance from the zero-size node to the goal + the origin to the goal
    get_tuple_distance(node, goal) + get_tuple_distance(goal, (0, 0))
}

fn get_shortest_path(nodes: &Vec<Node>, start: (u8, u8), finish: (u8, u8), ignoreNode: Option<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut previous: HashMap<(u8, u8), (u8, u8)> = HashMap::new();
    let mut distances: HashMap<(u8, u8), u8> = HashMap::new();
    for node in nodes.iter() {
        if node.x == start.0 && node.y == start.1 {
            distances.insert((node.x, node.y), 0);
        } else {
            distances.insert((node.x, node.y), u8::MAX);
        }
    }

    let mut pq = PriorityQueue::new();
    pq.push(start, Reverse(0));
    let mut end = (0, 0);

    while pq.len() > 0 {
        let mut node = pq.pop().unwrap().0;
        let actual_used = nodes.iter().find(|&n| n.x == node.0 && n.y == node.1).unwrap().used;
        let mut home_distance = *distances.get(&(node.0, node.1)).unwrap();
        let neighbors = get_neighbors(&nodes, node.0, node.1);
        for neighbor in neighbors {
            if neighbor.x == finish.0 && neighbor.y == finish.1 {
                end = node;
                previous.insert((neighbor.x, neighbor.y), (node.0, node.1));
                distances.insert((finish.0, finish.1), home_distance);
                break;
            } else if neighbor.size >= actual_used {
                if ignoreNode == Some((neighbor.x, neighbor.y)) {
                    continue;
                }

                let distance = distances.get_mut(&(neighbor.x, neighbor.y)).unwrap();
                // we add one for the current hop
                if *distance > home_distance + 1 {
                    *distance = home_distance + 1;
                    previous.insert((neighbor.x, neighbor.y), (node.0, node.1));
                    pq.push((neighbor.x, neighbor.y), Reverse(*distance));
                }
            }
        }
    }

    let mut results = vec![(finish.0, finish.1)];
    let mut previous_node = previous.get(&(finish.0, finish.1));
    while previous_node != None {
        let node = *previous_node.unwrap();

        if node != start {
            results.push(node);
        }

        previous_node = previous.get(&(node.0, node.1));
    }
    results.push((start.0, start.1));
    results.reverse();
    
    results
}