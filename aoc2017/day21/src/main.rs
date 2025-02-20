use std::collections::HashMap;
use std::fs::read_to_string;

use num::integer::Roots;


fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut rules = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split(" => ").collect();
        let mut rule_input = vec![];
        for part in parts[0].chars() {
            if part == '.' {
                rule_input.push(false);
            } else if part == '#' {
                rule_input.push(true);
            } // anything else (/) is ignored
        }

        let mut rule_output = vec![];
        for part in parts[1].chars() {
            if part == '.' {
                rule_output.push(false);
            } else if part == '#' {
                rule_output.push(true);
            } // anything else (/) is ignored
        }

        for _ in 1..=4 {
            if rule_input.len() == 4 {
                let rotation = rotate_size_2(&rule_input);
                rule_input = rotation.clone();
                rules.insert(rotation.clone(), rule_output.clone());

                let mut flip = flip_horizontal_size_2(&rotation);
                rules.insert(flip, rule_output.clone());
                flip = flip_vertical_size_2(&rotation);
                rules.insert(flip, rule_output.clone());
            } else {
                // we only have rules of size 4 or 9
                let rotation = rotate_size_3(&rule_input);
                rule_input = rotation.clone();
                rules.insert(rotation.clone(), rule_output.clone());

                let mut flip = flip_horizontal_size_3(&rotation);
                rules.insert(flip, rule_output.clone());
                flip = flip_vertical_size_3(&rotation);
                rules.insert(flip, rule_output.clone());
            }
        }
    }

    /* Initial grid:
        .#.
        ..#
        ###
    */
    let mut grid = vec![false, true, false, false, false, true, true, true, true];
    let iterations = 18;
    for iteration in 0..iterations {
        let step: usize = if grid.len() % 2 == 0 { 2 } else { 3 };
        let size = step.pow(2);
        let squares = grid.len() / size;
        let squares_per_row = squares.sqrt();

        let mut new_grid = vec![];
        let mut row = 0;
        for square_index in 0..squares {
            let mut square = vec![];
            for row in 0..step {
                let mut start = (row * squares_per_row * step);
                if square_index > squares_per_row - 1 {
                    // advance to the right row
                    start += ((square_index / squares_per_row) * step * step * squares_per_row);
                } 

                if square_index % squares_per_row != 0 {
                    let mut offset = square_index % squares_per_row;
                    if offset == 0 {
                        // to handle row 1
                        offset = square_index;
                    }

                    start += offset * step;
                }

                for column in 0..step {
                    square.push(grid[start + column]);
                }    
            }

            new_grid.push(rules.get(&square).unwrap());
        }

        if new_grid.len() == 1 {
            grid = new_grid[0].clone();
        } else {
            // we have to interleave the squares into a grid
            let new_grid_size = new_grid[0].len();
            let new_grid_len = new_grid.len() * new_grid_size;
            let new_grid_square_len = new_grid_size.sqrt();
            let new_grid_row_len = new_grid_size.sqrt();

            let mut constructed_grid = vec![false; new_grid_len];
            let mut grid_index = 0;
            let mut grid_square_index = 0;
            let mut grid_row_index = 0;
            let mut lights_on = 0;

            while grid_index < new_grid_len {
                for grid_column_index in 0..new_grid_row_len {
                    constructed_grid[grid_index] = new_grid[grid_square_index][(grid_row_index * new_grid_square_len) + grid_column_index];
                    if constructed_grid[grid_index] {
                        lights_on += 1;
                    }

                    grid_index += 1;
                }
    
                grid_square_index += 1;
                
                if grid_square_index % squares_per_row == 0 {
                    grid_row_index += 1;
                    if grid_row_index == new_grid_row_len {
                        grid_row_index = 0;
                    } else {
                        grid_square_index -= squares_per_row;
                    }
                }
            }

            println!("{}: {}", iteration + 1, lights_on);
            grid = constructed_grid;
        }
    }
}

fn flip_horizontal_size_2(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and flip
    result[0] = grid[1]; // starting in the upper-left
    result[1] = grid[0];
    result[2] = grid[3];
    result[3] = grid[2]; // lower-right
    
    result
}

fn flip_vertical_size_2(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and flip
    result[0] = grid[2]; // starting in the upper-left
    result[1] = grid[3];
    result[2] = grid[0];
    result[3] = grid[1]; // lower-right
    
    result
}

fn rotate_size_2(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and rotate
    result[0] = grid[1]; // starting in the upper-left
    result[1] = grid[3];
    result[2] = grid[0];
    result[3] = grid[2]; // lower-right
    
    result
}

fn flip_horizontal_size_3(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and flip
    result[0] = grid[2]; // starting in the upper-left
    result[1] = grid[1]; // middle doesn't move
    result[2] = grid[0];
    result[3] = grid[5];
    result[4] = grid[4]; // middle doesn't move
    result[5] = grid[3];
    result[6] = grid[8];
    result[7] = grid[7]; // middle doesn't move
    result[8] = grid[6]; // lower-right
    
    result  
}

fn flip_vertical_size_3(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and flip
    result[0] = grid[6]; // starting in the upper-left
    result[1] = grid[7]; 
    result[2] = grid[8];
    result[3] = grid[3]; // middle doesn't move
    result[4] = grid[4]; // middle doesn't move
    result[5] = grid[5]; // middle doesn't move
    result[6] = grid[0];
    result[7] = grid[1];
    result[8] = grid[2]; // lower-right
    
    result  
}

fn rotate_size_3(grid: &Vec<bool>) -> Vec<bool> {
    let mut result = vec![false; grid.len()];
    
    // we manually go in order and rotate
    result[0] = grid[2]; // starting in the upper-left
    result[1] = grid[5];
    result[2] = grid[8];
    result[3] = grid[1];
    result[4] = grid[4]; // middle doesn't move
    result[5] = grid[7];
    result[6] = grid[0];
    result[7] = grid[3];
    result[8] = grid[6]; // lower-right
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_horizontal_size_2_works() {
        let original = vec![true, false, false, true];
        let mut result = flip_horizontal_size_2(&original);
        assert_eq!(result, vec![false, true, true, false]);

        // two times is back to where we started
        result = flip_horizontal_size_2(&result);
        assert_eq!(result, original);
    }

    #[test]
    fn flip_vertical_size_2_works() {
        let original = vec![true, true, false, false];
        let mut result = flip_vertical_size_2(&original);
        assert_eq!(result, vec![false, false, true, true]);

        // two times is back to where we started
        result = flip_vertical_size_2(&result);
        assert_eq!(result, original);
    }

    #[test]
    fn rotate_size_2_works() {
        let original = vec![true, false, false, false];
        let mut result = rotate_size_2(&original);
        assert_eq!(result, vec![false, false, true, false]);
        
        result = rotate_size_2(&result);
        assert_eq!(result, vec![false, false, false, true]);
        
        result = rotate_size_2(&result);
        assert_eq!(result, vec![false, true, false, false]);
        
        // four times is back to where we started
        result = rotate_size_2(&result);
        assert_eq!(result, vec![true, false, false, false]);
    }

    #[test]
    fn flip_horizontal_size_3_works() {
        let original = vec![true, true, true, false, false, true, true, false, false];
        let mut result = flip_horizontal_size_3(&original);
        assert_eq!(result, vec![true, true, true, true, false, false, false, false, true]);

        // two times is back to where we started
        result = flip_horizontal_size_3(&result);
        assert_eq!(result, original);
    }

    #[test]
    fn flip_vertical_size_3_works() {
        let original = vec![true, true, true, false, false, true, false, false, false];
        let mut result = flip_vertical_size_3(&original);
        assert_eq!(result, vec![false, false, false, false, false, true, true, true, true]);

        // two times is back to where we started
        result = flip_vertical_size_3(&result);
        assert_eq!(result, original);
    }

    #[test]
    fn rotate_size_3_works() {
        let original = vec![true, true, false, false, false, true, true, true, true];
        let mut result = rotate_size_3(&original);
        assert_eq!(result, vec![false, true, true, true, false, true, true, false, true]);

        result = rotate_size_3(&result);
        assert_eq!(result, vec![true, true, true, true, false, false, false, true, true]);

        result = rotate_size_3(&result);
        assert_eq!(result, vec![true, false, true, true, false, true, true, true, false]);

        // four times is back to where we started
        result = rotate_size_3(&result);
        assert_eq!(result, original);
    }
}