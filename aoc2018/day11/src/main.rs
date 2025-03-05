const GRID_SIZE: usize = 300;
const SERIAL_NUMBER: u64 = 7857;

/* Part 1
fn main() {
    let mut grid: [i32; GRID_SIZE * GRID_SIZE] = [0; GRID_SIZE * GRID_SIZE];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            grid[((y - 1) * GRID_SIZE) + (x - 1)] = calculate_power_level(x as u64, y as u64, SERIAL_NUMBER);
        }
    }

    let mut max = i32::MIN;
    let mut max_coordinates = (0, 0);
    for y in 1..=(GRID_SIZE - 2) { 
        for x in 1..=(GRID_SIZE - 2) {
            let mut current_x = x - 1;
            let mut current_y = y - 1;
            let mut result = 0;

            for _ in 0..3 {
                result += grid[(current_y * GRID_SIZE) + current_x];
                result += grid[(current_y * GRID_SIZE) + current_x + 1];
                result += grid[(current_y * GRID_SIZE) + current_x + 2];

                current_y += 1;
            }

            if result > max {
                max = result;
                max_coordinates = (x, y);
            }
        }
    }

    println!("Max Coordinates: {},{}", max_coordinates.0, max_coordinates.1); // Part 1
}
*/

fn main() {
    let mut grid: [i32; GRID_SIZE * GRID_SIZE] = [0; GRID_SIZE * GRID_SIZE];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            grid[((y - 1) * GRID_SIZE) + (x - 1)] = calculate_power_level(x as u64, y as u64, SERIAL_NUMBER);
        }
    }

    let mut max_dimension = 0;
    let mut max = i32::MIN;
    let mut max_coordinates = (0, 0);

    // we can skip squares of size 1, since the max is 4 (and definitely not the highest)
    // we can skip 2 since 16 is the max and we know size 3 is bigger
    // we can skip 3 because that's the answer to Part 1
    for square_size in 4..=GRID_SIZE {
        let grid_max = 300 - (square_size - 1);
        for y in 1..=grid_max { 
            for x in 1..=grid_max {
                let mut current_x = x - 1;
                let mut current_y = y - 1;
                let mut result = 0;
    
                for _ in 0..square_size {
                    for column_index in 0..square_size {
                        result += grid[(current_y * GRID_SIZE) + current_x + column_index];
                    }

                    current_y += 1;
                }
    
                if result > max {
                    max = result;
                    max_coordinates = (x, y);
                    max_dimension = square_size;
                }
            }
        }
    }

    println!("Max value: {}", max);
    println!("Max: {},{},{}", max_coordinates.0, max_coordinates.1, max_dimension);
}

fn calculate_power_level(x: u64, y: u64, serial_number: u64) -> i32 {
    let rack_id = x + 10;
    let mut result = rack_id * y;
    result += serial_number;
    result *= rack_id;
    let result_str = result.to_string();
    let mut hundreds = 0;
    if result_str.len() > 2 {
        // 48 is the ASCII code for '0'
        hundreds = (result_str.chars().rev().nth(2).unwrap() as i32) - 48;
    }

    hundreds - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_power_level() {
        assert_eq!(calculate_power_level(3, 5, 8), 4);
        assert_eq!(calculate_power_level(122, 79, 57), -5);
        assert_eq!(calculate_power_level(217, 196, 39), 0);
        assert_eq!(calculate_power_level(101, 153, 71), 4);
    }
}