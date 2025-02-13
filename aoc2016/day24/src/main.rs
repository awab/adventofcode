use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;
use priority_queue::PriorityQueue;

const GRID_WIDTH: usize = 179;
const GRID_HEIGHT: usize = 43;
const MAX_COORDINATE: char = '7';

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut grid: [bool; GRID_WIDTH * GRID_HEIGHT] = [false; GRID_WIDTH * GRID_HEIGHT];
    let mut coordinates = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    for line in lines {
        x = 0;
        for ch in line.chars() {
            grid[(y * GRID_WIDTH) + x] = if ch == '#' { true } else { false };
            if ch.is_digit(10) {
                coordinates.insert(ch, (x, y));
            }
            x += 1;
        }

        y += 1;
    }

    /*
    draw_grid(&grid, &coordinates, None);
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let zero = coordinates.get(&'4').unwrap();
    let one = coordinates.get(&'6').unwrap();
    let path = find_shortest_path(&grid, (zero.0, zero.1), (one.0, one.1));
    draw_grid(&grid, &coordinates, Some(&path));
    */

    let len = find_shortest_path_length_through_all_points(&grid, &coordinates);
    println!("Length: {}", len);
}

fn draw_grid(grid: &[bool], coordinates: &HashMap<char, (usize, usize)>, path: Option<&Vec<(usize, usize)>>) {
    let mut points = vec![];
    for i in '0'..=MAX_COORDINATE {
        points.push(*coordinates.get(&i).unwrap());
    }

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let mut ch = if grid[(y * GRID_WIDTH) + x] { '#' } else { '.' };
            if points.contains(&(x, y)) {
                ch = char::from_digit(points.iter().position(|&key| key == (x, y)).unwrap() as u32, 10).unwrap();
            } else if path != None {
                if path.unwrap().contains(&(x, y)) {
                    ch = 'X';
                }
            }

            print!("{}", ch);
        }
        println!("");
    }
}

fn get_neighbors(grid: &[bool], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut results = vec![];

    // up
    if y > 0 {
        if !grid[((y - 1) * GRID_WIDTH) + x] {
            results.push((x, y - 1));
        }
    }

    // right
    if x > 0 && x % GRID_WIDTH != 0 {
        if !grid[(y * GRID_WIDTH) + x + 1] {
            results.push((x + 1, y));
        }
    }

    // down
    if y < GRID_HEIGHT - 1 {
        if !grid[((y + 1) * GRID_WIDTH) + x] {
            results.push((x, y + 1));
        }
    }

    // left
    if x != 0 {
        if !grid[(y * GRID_WIDTH) + x - 1] {
            results.push((x - 1, y));
        }
    }

    results
}

fn find_shortest_path(grid: &[bool], start: (usize, usize), finish: (usize, usize)) -> Vec<(usize, usize)> {
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if !grid[(y * GRID_WIDTH) + x] {
                distances.insert((x, y), usize::MAX);
            }
        }
    }

    distances.insert((start.0, start.1), 0);

    let mut pq = PriorityQueue::new();
    pq.push(start, Reverse(0));
    
    while pq.len() > 0 {
        let mut node = pq.pop().unwrap().0;
        let mut home_distance = *distances.get(&(node.0, node.1)).unwrap();
        let neighbors = get_neighbors(&grid, node.0, node.1);
        for neighbor in neighbors {
            if neighbor.0 == finish.0 && neighbor.1 == finish.1 {
                previous.insert((neighbor.0, neighbor.1), (node.0, node.1));
                distances.insert((finish.0, finish.1), home_distance);
                break;
            } else {
                let distance = distances.get_mut(&(neighbor.0, neighbor.1)).unwrap();
                
                // we add one for the current hop
                if *distance > home_distance + 1 {
                    *distance = home_distance + 1;
                    previous.insert((neighbor.0, neighbor.1), (node.0, node.1));
                    pq.push((neighbor.0, neighbor.1), Reverse(*distance));
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

fn find_shortest_path_length_through_all_points(grid: &[bool], coordinates: &HashMap<char, (usize, usize)>) -> usize {
    let mut points = vec![];
    for i in '1'..=MAX_COORDINATE {
        points.push(*coordinates.get(&i).unwrap());
    }

    let zero = *coordinates.get(&'0').unwrap();

    let mut cache = HashMap::new();
    let mut shortest = usize::MAX;
    for permutation in points.clone().into_iter().permutations(points.len()) {
        let mut last = zero;
        let mut length = 0;
        for point in permutation {
            let mut current_length = cache.get(&(last.0, last.1, point.0, point.1));
            if current_length == None {
                let current_length = find_shortest_path(grid, (last.0, last.1), (point.0, point.1)).len() - 1;
                length += current_length;
                cache.insert((last.0, last.1, point.0, point.1), current_length);
            } else {
                length += current_length.unwrap();                
            }

            if length >= shortest {
                continue;
            }

            last = point;
        }

        // Part 2: return to 0
        length += find_shortest_path(grid, (last.0, last.1), (zero.0, zero.1)).len() - 1;
        
        if length < shortest {
            shortest = length;
        }
    }

    shortest
}
