use std::cmp::Reverse;

use priority_queue::PriorityQueue;

/* Part 1
fn main() {
    let mut pq = PriorityQueue::new();
    let mut x = 1;
    let mut y = 1;
    let target_x = 31;
    let target_y = 39;

    pq.push((x, y), Reverse(0));

    while pq.len() > 0 {
        let item = pq.pop().unwrap();
        let (x, y) = item.0;
        let steps = item.1;

        if x == target_x && y == target_y {
            // since we use a priority queue, as soon as we get there, we're done
            println!("Shortest distance: {} steps", steps.0);
            return;
        }

        // we check for walls in all cardinal directions and add moves accordingly
        // up
        let up_check = y - 1;
        if up_check >= 0 && !is_wall(x, up_check) {
            pq.push((x, up_check), Reverse(steps.0 + 1));
        }

        // right
        if !is_wall(x + 1, y) {
            pq.push((x + 1, y), Reverse(steps.0 + 1));
        }

        // down
        if !is_wall(x, y + 1) {
            pq.push((x, y + 1), Reverse(steps.0 + 1));
        }

        // left
        let left_check = x - 1;
        if left_check >= 0 && !is_wall(x - 1, y) {
            pq.push((x - 1, y), Reverse(steps.0 + 1));
        }
    }
}
*/

fn main() {
    let mut pq = PriorityQueue::new();
    let mut x = 1;
    let mut y = 1;
    let mut results = vec![];

    pq.push((x, y), Reverse(0));

    while pq.len() > 0 {
        let item = pq.pop().unwrap();
        let (x, y) = item.0;
        let steps = item.1;

        if steps.0 <= 50 && !results.contains(&(x, y)) {
            results.push((x, y));
        } else if steps.0 > 50 {
            // we quit as soon as we exceed 50 steps
            continue;
        }

        // we check for walls in all cardinal directions and add moves accordingly
        // up
        let up_check = y - 1;
        if up_check >= 0 && !is_wall(x, up_check) {
            pq.push((x, up_check), Reverse(steps.0 + 1));
        }

        // right
        if !is_wall(x + 1, y) {
            pq.push((x + 1, y), Reverse(steps.0 + 1));
        }

        // down
        if !is_wall(x, y + 1) {
            pq.push((x, y + 1), Reverse(steps.0 + 1));
        }

        // left
        let left_check = x - 1;
        if left_check >= 0 && !is_wall(x - 1, y) {
            pq.push((x - 1, y), Reverse(steps.0 + 1));
        }
    }

    println!("Total locations <= 50 steps away: {}", results.len());
}

fn is_wall(x: i32, y: i32) -> bool {
    const magic_number: i32 = 1362;
    let mut value = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
    value += magic_number;
    (0..32).map (|n| (value >> n) & 1).sum::<i32>() % 2 != 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_wall_works() {
        // (1, 1) is our starting space and not a wall
        assert_eq!(is_wall(1, 1), false);

        // (2, 1) == 1378 = 0b10101100010 == a wall
        assert_eq!(is_wall(2, 1), true);
    }
}