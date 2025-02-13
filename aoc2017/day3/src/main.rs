fn main() {
    let target: u32 = 347991;

    /* Part 1
    let mut index: i32 = 3;
    while index.pow(2) < target {
        index += 2;
    }

    // the lower-right corner of the part of the square containing our number is the square of index
    // we have to go back one to figure out where we started
    let mut position = (index - 2).pow(2);
    let difference = target - position + 1; // add 1 since the end of the row is part of it
    let position_in_row = difference % index;
    let distance_from_center = (index / 2) - position_in_row;
    let total_distance = distance_from_center.abs() + (index / 2);  // it's other offset is half the length of our index

    println!("Total distance: {}", total_distance);
    */

    // a big enough grid that we know we can start at the middle and brute force it
    let mut grid: [u32; 1000 * 1000] = [0; 1000 * 1000];
    let mut x = 500;
    let mut y = 500;
    let mut direction = 'L';
    let mut index = 1;
    let mut len = 3;
    set_grid(&mut grid, x, y, 1);
    x += 1;
    set_grid(&mut grid, x, y, 1);
    y -= 1;

    loop {
        let value = get_calculated_value(&grid, x, y);
        if value > target {
            println!("First value: {}", value);
            return;
        }

        set_grid(&mut grid, x, y, value);
        if index % len == 0 {
            direction = change_direction(direction);
            index = 1;

            if direction == 'R' || direction == 'L' {
                // if the new direction is 'U', we increment the length of the row
                len += 1;
            }
        }

        if direction == 'U' {
            y -= 1;
        } else if direction == 'L' {
            x -= 1;
        } else if direction == 'D' {
            y += 1;
        } else if direction == 'R' {
            x += 1;
        }

        index += 1;
    }
}

fn get_grid(grid: &[u32], x: usize, y: usize) -> u32 {
    grid[(y * 1000) + x]
}

fn set_grid(grid: &mut [u32], x: usize, y: usize, value: u32) {
    grid[(y * 1000) + x] = value;
}

fn get_calculated_value(grid: &[u32], x: usize, y: usize) -> u32 {
    let mut total = 0;

    total += get_grid(&grid, x, y - 1);        // up
    total += get_grid(&grid, x + 1, y - 1);    // up-right
    total += get_grid(&grid, x + 1, y);        // right
    total += get_grid(&grid, x + 1, y + 1);    // down-right
    total += get_grid(&grid, x, y + 1);        // down
    total += get_grid(&grid, x - 1, y + 1);    // down-left
    total += get_grid(&grid, x - 1, y);        // left
    total += get_grid(&grid, x - 1, y - 1);    // up-right

    total
}

fn change_direction(direction: char) -> char {
    match direction {
        'U' => 'L',
        'L' => 'D',
        'D' => 'R',
        'R' => 'U',
        _ => panic!("Invalid direction!"),
    }
}