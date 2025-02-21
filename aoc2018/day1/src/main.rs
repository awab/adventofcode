use std::fs::read_to_string;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut total: i64 = 0;
    for line in lines {
        let number_str = line;
        let mut number = 0;
        if number_str.starts_with("+") {
            number = number_str[1..=(number_str.len() - 1)].parse::<i32>().unwrap();
        } else {
            number = number_str.parse::<i32>().unwrap();    
        }

        total += number as i64;
    }    

    println!("Total: {}", total);
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut totals: Vec<i64> = vec![];
    let mut total: i64 = 0;
    loop {
        for line in &lines {
            let number_str = line;
            let mut number = 0;
            if number_str.starts_with("+") {
                number = number_str[1..=(number_str.len() - 1)].parse::<i32>().unwrap();
            } else {
                number = number_str.parse::<i32>().unwrap();    
            }

            total += number as i64;
            if totals.contains(&total) {
                println!("First repeat total found: {}", total);
                return;
            }

            totals.push(total);
        }    
    }
}