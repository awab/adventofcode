const PLAYER_COUNT: usize = 423;
const MARBLE_LAST_POINTS: usize = 7194400; // Part 1: 71944
// this runs in a reasonable time, so we just brute-force Part 2

fn main() {
    let mut marbles = vec![];
    let mut marble = 0;
    let mut marble_index = 0;
    let mut player_index = 0;
    let mut scores = [0; PLAYER_COUNT];
    while marble <= MARBLE_LAST_POINTS {
        if marble != 0 && marble % 23 == 0 {
            scores[player_index] += marble;
            for i in 0..7 {
                if marble_index == 0 {
                    marble_index = marbles.len() - 1;
                } else {
                    marble_index -= 1;
                } 
            }

            scores[player_index] += marbles[marble_index];
            marbles.remove(marble_index);

            if marble_index >= marbles.len() {
                marble_index = 0;
            }
        } else {
            if marbles.len() == 0 {
                marble_index = 0;
            } else {
                marble_index += 1;
                if marble_index == marbles.len() {
                    marble_index = 0;
                }
                marble_index += 1;
            }

            marbles.insert(marble_index, marble);
        }

        marble += 1;
        player_index += 1;
        if player_index == PLAYER_COUNT {
            player_index = 0;
        }
    }

    let max_score = scores.iter().max_by(|a, b| a.cmp(&b)).unwrap();
    println!("Max Score: {}", max_score);
}
