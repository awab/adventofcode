use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    let grid_width = lines[0].len();
    let grid_height = lines.len();
    let mut grid = vec![];
    for line in &lines {
        for ch in line.chars() {
            grid.push(ch);
        }
    }

    // todo: move(&grid, index, direction)
    // todo: deal with lines crossing

    // start at the first |
    let mut index = grid.iter().position(|x| *x == '|').unwrap();
    let mut direction = 'D';
    let mut letters = vec![];
    let mut steps = 0;
    loop {
        // try to move
        if grid[index] == ' ' {
            // we're done!
            break;
        }
        else if grid[index] == '|' {
            // we go straight
            if direction == 'D' {
                index += grid_width;
            } else if direction == 'U' {
                index -= grid_width;
            } else if direction == 'R' {
                // crossing over
                index += 1;
            } else if direction == 'L' {
                // crossing over
                index -= 1;
            }
        } else if grid[index] == '-' {
            // we go straight
            if direction == 'R' {
                index += 1;
            } else if direction == 'L' {
                index -= 1;
            } else if direction == 'D' {
                // crossing over
                index += grid_width;
            } else if direction == 'U' {
                // crossing over
                index -= grid_width;
            } 
        } else if grid[index] == '+' {
            // we need to turn
            let mut turned = false;

            // try left
            if direction != 'R' && index % grid_width != 0 {
                if grid[index - 1] == '-' || grid[index - 1].is_alphanumeric() {
                    turned = true;
                    direction = 'L';
                    index -= 1;
                }
            }

            // try right
            if !turned && direction != 'L' {
                if index != grid.len() - 1 && (index + 1) % grid_width != 0 {
                    if grid[index + 1] == '-' || grid[index + 1].is_alphanumeric() {
                        turned = true;
                        direction = 'R';
                        index += 1;
                    }
                }
            }

            // try up
            if !turned && direction != 'D' {
                if index >= grid_width {
                    if grid[index - grid_width] == '|' || grid[index - grid_width].is_alphanumeric() {
                        turned = true;
                        direction = 'U';
                        index -= grid_width;
                    }
                }
            }

            // try down
            if !turned && direction != 'U' {
                if index < (grid_width * grid_height) - grid_width {
                    if grid[index + grid_width] == '|' || grid[index + grid_width].is_alphanumeric() {
                        turned = true;
                        direction = 'D';
                        index += grid_width;
                    }
                }
            }
        } else {
            // it's a letter we need to capture
            letters.push(grid[index]);

            if direction == 'D' {
                index += grid_width;
            } else if direction == 'U' {
                index -= grid_width;
            } else if direction == 'L' {
                index -= 1;
            } else if direction == 'R' {
                index += 1;
            }
        }

        steps += 1;
    }

    // Part 1
    for letter in letters {
        print!("{}", letter);
    }

    println!("");

    // Part 2
    println!("Steps: {}", steps);
}