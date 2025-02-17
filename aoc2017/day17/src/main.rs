/* Part 1
fn main() {
    let step_length = 316;
    let mut data = vec![0];
    let mut index = 0;
    let mut count = 1;
    for _ in 1..=2017 {
        index = (index + step_length) % data.len();
        
        if index == (data.len() - 1) {
            data.push(count);
        } else {
            data.insert(index + 1, count);
        }

        index += 1;
        count += 1;
    }

    let solution_index = data.iter().position(|x| *x == 2017).unwrap();
    println!("Solution: {}", data[solution_index + 1]);
}
*/

fn main() {
    let step_length = 316;
    let mut data = vec![0];
    let mut index = 0;
    let mut count = 1;
    let mut len = 1;
    let mut last_insert = 0;
    loop {
        index = (index + step_length) % len;
        
        if index == 0 {
            last_insert = count;
        }

        if count == 50000000 {
            println!("Solution: {}", last_insert);
            return;
        }

        len += 1;
        index += 1;
        count += 1;
    }

    println!("Solution: {}", data[1]);
}
