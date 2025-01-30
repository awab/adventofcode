use std::cmp::Reverse;

use priority_queue::PriorityQueue;

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    pub x: u8,
    pub y: u8,
    pub directions: String,
}

/* Part 1
fn main() {
    let input = String::from("qtetzkpl");
    let state = State {
        x: 0,
        y: 0,
        directions: String::from(""),
    };

    let mut pq = PriorityQueue::new();
    pq.push(state, Reverse(0));

    while pq.len() > 0 {
        let item = pq.pop().unwrap();
        let state = item.0;
        let move_number = item.1.0 + 1;

        if state.x == 3 && state.y == 3 {
            // we're done and since it's a priority queue, we should have the shortest path
            println!("Done.  Shortest path: {}", state.directions);
            return;
        }

        let digest = md5::compute(input.clone() + &state.directions);
        let digest_hex = format!("{:x}", digest);
        
        // up
        if state.y > 0 {
            let first_digit = digest_hex.chars().nth(0).unwrap();
            if first_digit == 'b' || first_digit == 'c' || first_digit == 'd' || first_digit == 'e' || first_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('U');
                new_state.y -= 1;
                pq.push(new_state, Reverse(move_number));
            }
        }

        // down
        if state.y < 3 {
            let second_digit = digest_hex.chars().nth(1).unwrap();
            if second_digit == 'b' || second_digit == 'c' || second_digit == 'd' || second_digit == 'e' || second_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('D');
                new_state.y += 1;
                pq.push(new_state, Reverse(move_number));
            }
        }

        // left
        if state.x > 0 {
            let third_digit = digest_hex.chars().nth(2).unwrap();
            if third_digit == 'b' || third_digit == 'c' || third_digit == 'd' || third_digit == 'e' || third_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('L');
                new_state.x -= 1;
                pq.push(new_state, Reverse(move_number));
            }
        }

        // right
        if state.x < 3 {
            let fourth_digit = digest_hex.chars().nth(3).unwrap();
            if fourth_digit == 'b' || fourth_digit == 'c' || fourth_digit == 'd' || fourth_digit == 'e' || fourth_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('R');
                new_state.x += 1;
                pq.push(new_state, Reverse(move_number));
            }
        }
    }
}
*/

fn main() {
    let input = String::from("qtetzkpl");
    let state = State {
        x: 0,
        y: 0,
        directions: String::from(""),
    };

    let mut pq = PriorityQueue::new();
    let mut longest_path = 0;
    pq.push(state, 0);

    while pq.len() > 0 {
        let item = pq.pop().unwrap();
        let state = item.0;
        let move_number = item.1 + 1;

        if state.x == 3 && state.y == 3 {
            // we're done and since it's a priority queue, we should have the shortest path
            if state.directions.len() > longest_path {
                longest_path = state.directions.len();
            }

            continue;
        }

        let digest = md5::compute(input.clone() + &state.directions);
        let digest_hex = format!("{:x}", digest);
        
        // up
        if state.y > 0 {
            let first_digit = digest_hex.chars().nth(0).unwrap();
            if first_digit == 'b' || first_digit == 'c' || first_digit == 'd' || first_digit == 'e' || first_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('U');
                new_state.y -= 1;
                pq.push(new_state, move_number);
            }
        }

        // down
        if state.y < 3 {
            let second_digit = digest_hex.chars().nth(1).unwrap();
            if second_digit == 'b' || second_digit == 'c' || second_digit == 'd' || second_digit == 'e' || second_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('D');
                new_state.y += 1;
                pq.push(new_state, move_number);
            }
        }

        // left
        if state.x > 0 {
            let third_digit = digest_hex.chars().nth(2).unwrap();
            if third_digit == 'b' || third_digit == 'c' || third_digit == 'd' || third_digit == 'e' || third_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('L');
                new_state.x -= 1;
                pq.push(new_state, move_number);
            }
        }

        // right
        if state.x < 3 {
            let fourth_digit = digest_hex.chars().nth(3).unwrap();
            if fourth_digit == 'b' || fourth_digit == 'c' || fourth_digit == 'd' || fourth_digit == 'e' || fourth_digit == 'f' {
                let mut new_state = state.clone();
                new_state.directions.push('R');
                new_state.x += 1;
                pq.push(new_state, move_number);
            }
        }
    }

    println!("Longest path: {}", longest_path);
}