use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(PartialEq, Clone, Copy)]
enum Flag {
    Clean,
    Weakened,
    Infected,
    Flagged
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let line_length = lines[0].len();
    let midpoint = (line_length / 2) as i32; // it's always odd in this puzzle
    
    let mut grid = HashMap::new();
    let mut x = -1 * midpoint;
    let mut y = -1 * midpoint;
    for line in lines {
        for ch in line.chars() {
            let infected = if ch == '.' { Flag::Clean } else { Flag::Infected };
            grid.insert((x, y), infected);

            x += 1;
        }

        x = -1 * midpoint;
        y += 1;
    }

    let mut current_position = (0, 0);
    let mut current_direction = 'U';

    // Part 1: let iterations = 10000;
    // for iteration in 0..iterations {

    let infection_burst_target = 10000000;
    let mut infection_bursts = 0;
    let mut actual_infections = 0;
    loop {
        infection_bursts += 1;
        let current_node = *grid.get(&current_position).unwrap();
        if current_node == Flag::Infected {
            current_direction = change_direction(&current_direction, 'R');
            grid.insert(current_position, Flag::Flagged);
        } else if current_node == Flag::Clean {
            current_direction = change_direction(&current_direction, 'L');
            grid.insert(current_position, Flag::Weakened);
        } else if current_node == Flag::Flagged {
            current_direction = reverse_direction(current_direction);
            grid.insert(current_position, Flag::Clean);
        } else if current_node == Flag::Weakened {
            // no direction change
            grid.insert(current_position, Flag::Infected);
            actual_infections += 1;
        }

        if infection_bursts == infection_burst_target {
            break;
        }

        current_position = move_position(current_position, current_direction);
        let new_node = grid.get(&current_position);
        if new_node == None {
            // initialize with a clean node
            grid.insert(current_position, Flag::Clean);
        }
    }

    println!("Total infection bursts: {}", infection_bursts);   // Part 1
    println!("Total infections: {}", actual_infections);        // Part 2
}

fn change_direction(current_direction: &char, new_direction: char) -> char {
    let mut result = '\0';
    if new_direction == 'R' {
        result = match current_direction {
            'U' => 'R',
            'R' => 'D',
            'D' => 'L',
            'L' => 'U',
            _ => panic!("Unknown direction {}", current_direction),
        };
    } else {
        result = match current_direction {
            'U' => 'L',
            'R' => 'U',
            'D' => 'R',
            'L' => 'D',
            _ => panic!("Unknown direction {}", current_direction),
        };
    }

    result
}


fn reverse_direction(current_direction: char) -> char {
    if current_direction == 'U' {
        return 'D';
    } else if current_direction == 'R' {
        return 'L';
    } else if current_direction == 'D' {
        return 'U';
    } else if current_direction == 'L' {
        return 'R';
    }

    return '\0';
}

fn move_position(current_position: (i32, i32), current_direction: char) -> (i32, i32) {
    let mut new_position = current_position.clone();
    if current_direction == 'U' {
        new_position.1 -= 1;
    } else if current_direction == 'R' {
        new_position.0 += 1;
    } else if current_direction == 'D' {
        new_position.1 += 1;
    } else if current_direction == 'L' {
        new_position.0 -= 1;
    }

    new_position
}