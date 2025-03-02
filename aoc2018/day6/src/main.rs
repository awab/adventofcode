use std::collections::HashMap;
use std::fs::read_to_string;

use kdtree::KdTree;
use num_traits::Float;

/* Part 1
fn main() {
    let filename = "input.txt";
    let mut lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut coordinates = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(", ").collect();
        coordinates.push((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
    }

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let mut neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut tree_list = vec![];
    for index in 0..coordinates.len() {
        tree_list.push([coordinates[index].0 as f32, coordinates[index].1 as f32]);
    }
    
    let dimensions = 2;
    let mut kdtree = KdTree::new(dimensions);
    for index in 0..tree_list.len() {
        kdtree.add(&tree_list[index], index);
    }

    let mut infinite = vec![];

    // we have to go through each non-corner to make sure we find the max area
    for index in 0..coordinates.len() {
        let coordinate = coordinates[index];
        find_area(&mut grid, &coordinates, &mut neighbors, &kdtree, &mut infinite, index);
    }

    let mut areas: HashMap<usize, i32> = HashMap::new();
    for key in grid.keys() {
        let value = grid.get(key).unwrap();
        if *value == -1 || infinite.contains(&(*value as usize)) {
            continue;
        }

        *areas.entry(*value as usize).or_insert(0) += 1;
    }
    
    let max_area = areas.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    println!("Areas: {}", max_area);
}

fn find_area(grid: &mut HashMap<(i32, i32), i32>, coordinates: &Vec<(i32, i32)>, neighbors: &mut HashMap<usize, Vec<usize>>, kdtree: &KdTree<f32, usize, &[f32; 2]>, infinite: &mut Vec<usize>, index: usize) {
    let home = coordinates[index];
    grid.insert(home, index as i32);

    let mut queue = vec![];
    
    let mut first_neighbors = get_valid_neighbors(&grid, &coordinates, home);
    queue.append(&mut first_neighbors);

    while queue.len() > 0 {
        let coordinate = queue.pop().unwrap(); // this takes from the back but order doesn't matter

        // visit the current node
        let shortest_distances = find_shortest_distances(coordinate, &coordinates, &kdtree);
        if shortest_distances.len() == 1 {
            if shortest_distances[0].1 != index {
                // we found a neighbor, let's mark it as such and move on without checking for more neighbors
                /*
                let current_neighbors = neighbors.entry(index).or_insert(vec![]);
                if !current_neighbors.contains(&index) {
                    current_neighbors.push(index);
                }
                */
                // if we bled into another area, we abort without getting more neighbors
                continue;
            } else if shortest_distances[0].0 > 1000 {
                // we've found an infinite region
                infinite.push(index);
                return;
            } else {
                grid.insert(coordinate, shortest_distances[0].1 as i32);
            }
        } else {
            grid.insert(coordinate, -1); // -1 indicates multiple closest neighbors
            continue;
        }

        let mut new_neighbors = get_valid_neighbors(&grid, &coordinates, coordinate);
        queue.append(&mut new_neighbors);
    }
}

fn find_shortest_distances(point: (i32, i32), coordinates: &Vec<(i32, i32)>, kdtree: &KdTree<f32, usize, &[f32; 2]>) -> Vec<(i32, usize)> {
    let tree_point = [point.0 as f32, point.1 as f32];
    let nearest = kdtree.nearest(&tree_point, 2, &manhattan).unwrap();

    // the 2 nearest points are the same distance away
    if nearest[0].0 == nearest[1].0 {
        return vec![(0, 1), (1, 1)]; // indexes don't matter when there's more than one
    }

    vec![(nearest[0].0 as i32, *nearest[0].1)]
}
*/

fn main() {
    let filename = "input.txt";
    let mut lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut coordinates = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(", ").collect();
        coordinates.push((parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap()));
    }

    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
    let mut index = 0;

    let grid_size = 1000;
    for x in (-1 * grid_size)..=grid_size {
        for y in (-1 * grid_size)..=grid_size {
            let coordinate = (x, y);
            let distance = find_total_distance(coordinate, &coordinates);
            if distance < 10000 {
                grid.insert(coordinate, true);
            } else {
                grid.insert(coordinate, false);
            }
        }    
    }

    let mut total = 0;
    for key in grid.keys() {
        let in_region = grid.get(key).unwrap();
        if *in_region {
            total += 1;
        }
    }

    println!("Total: {}", total);
}

fn find_total_distance(point: (i32, i32), coordinates: &Vec<(i32, i32)>) -> i32 {
    let mut total = 0;

    for coordinate in coordinates {
        total += (point.0 - coordinate.0).abs() + (point.1 - coordinate.1).abs()
    }

    total
}

fn get_valid_neighbors(grid: &HashMap<(i32, i32), i32>, coordinates: &Vec<(i32, i32)>, point: (i32, i32)) -> Vec<(i32, i32)> {
    let mut results = vec![];
    let DIRECTIONS: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    for direction in DIRECTIONS {
        let new_point = (point.0 + direction.0, point.1 + direction.1);
        if !grid.contains_key(&new_point) {
            results.push(new_point);
        }
    }

    results
}

pub fn manhattan<T: Float>(a: &[T], b: &[T]) -> T {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| ((*x) - (*y)).abs() + ((*x) - (*y)).abs())
        .fold(T::zero(), ::std::ops::Add::add)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid_neighbors() {
        let mut grid = HashMap::new();
        let coordinates = vec![(3, 0)];
        let mut neighbors = get_valid_neighbors(&grid, &coordinates, (1, 1));
        assert_eq!(neighbors, vec![(0, 1), (1, 0), (2, 1), (1, 2)]);

        // it should block the coordinate at (3, 0)
        neighbors = get_valid_neighbors(&grid, &coordinates, (3, 1));
        assert_eq!(neighbors, vec![(2, 1), (4, 1), (3, 2)]);

        grid.insert((2, 1), -1);
        // it should skip the node that's already done
        neighbors = get_valid_neighbors(&grid, &coordinates, (3, 1));
        assert_eq!(neighbors, vec![(4, 1), (3, 2)]);
    }
}