use std::fs::File;
use std::io::Read;

const GRID_SIZE: i16 = 100;

fn main() {
    let filename = "input.txt";
    let mut data = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut data));

    // make sure the corners start on
    data[0] = b'#';
    data[99] = b'#';
    data[9900] = b'#';
    data[9999] = b'#';

    let mut new_state = update_state(data);
    for _ in 1..100 {
        new_state = update_state(new_state);
    }

    let mut number_on = 0;
    for byte in new_state {
        if byte == b'#' {
            number_on += 1;
        }
    }

    println!("Number on at end: {}", number_on);
}

fn update_state(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];
    let grid_max = GRID_SIZE * GRID_SIZE;
    for index in 0..grid_max {
        let x_value: i16 = index % GRID_SIZE;
        let y_value: i16 = index / GRID_SIZE;

        if (x_value == 0 && y_value == 0) ||
           (x_value == (GRID_SIZE - 1) && y_value == 0) ||
           (x_value == 0 && y_value == (GRID_SIZE - 1)) ||
           (x_value == (GRID_SIZE - 1) && y_value == (GRID_SIZE - 1)) {
            result.push(b'#');
            continue;
            // the corners are always on
        }

        let mut number_on = 0;
        
        let neighbor1 = ((y_value - 1) * GRID_SIZE) + (x_value - 1);
        if (x_value - 1) >= 0 && (y_value - 1) >= 0 && neighbor1 >= 0 && neighbor1 < grid_max {
            if data[neighbor1 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor2 = ((y_value - 1) * GRID_SIZE) + x_value;
        if (y_value - 1) >= 0 && neighbor2 >= 0 && neighbor2 < grid_max {
            if data[neighbor2 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor3 = ((y_value - 1) * GRID_SIZE) + (x_value + 1);
        if (x_value + 1) < GRID_SIZE && (y_value - 1) >= 0 && neighbor3 >= 0 && neighbor3 < grid_max {
            if data[neighbor3 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor4 = (y_value * GRID_SIZE) + (x_value - 1);
        if (x_value - 1) >= 0 && neighbor4 >= 0 && neighbor4 < grid_max {
            if data[neighbor4 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor5 = (y_value * GRID_SIZE) + (x_value + 1);
        if (x_value + 1) < GRID_SIZE && neighbor5 >= 0 && neighbor5 < grid_max {
            if data[neighbor5 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor6 = ((y_value + 1) * GRID_SIZE) + (x_value - 1);
        if (x_value - 1) >= 0 && (y_value + 1) < 100 && neighbor6 >= 0 && neighbor6 < grid_max {
            if data[neighbor6 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor7 = ((y_value + 1) * GRID_SIZE) + x_value;
        if (y_value + 1) < 100 && neighbor7 >= 0 && neighbor7 < grid_max {
            if data[neighbor7 as usize] == b'#' {
                number_on += 1;
            }
        }

        let neighbor8 = ((y_value + 1) * GRID_SIZE) + (x_value + 1);
        if (x_value + 1) < GRID_SIZE && (y_value + 1) < 100 && neighbor8 >= 0 && neighbor8 < grid_max {
            if data[neighbor8 as usize] == b'#' {
                number_on += 1;
            }
        }

        if data[index as usize] == b'#' {
            // light is on, keep it on if 2 or 3 neighbors are lit
            if number_on == 2 || number_on == 3 {
                result.push(b'#');
            } else {
                result.push(b'.');
            }
        } else {
            // light if off, turn it on if 3 neighbors are lit
            if number_on == 3 {
                result.push(b'#');
            } else {
                result.push(b'.');
            }
        }
    }
    
    result
}

fn print_table(data: Vec<u8>) {
    let mut index = 0;
    for byte in data {
        print!("{}", byte as char);
        
        index += 1;
        if index % GRID_SIZE == 0 {
            println!("");
        }
    }
}