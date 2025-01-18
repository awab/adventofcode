use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut chars: HashMap<usize, HashMap<char, u16>> = HashMap::new();
    for line in lines {
        for index in 0..line.len() {
            let list = chars.entry(index).or_insert(HashMap::new());
            let ch = line.chars().nth(index).unwrap();
            *list.entry(ch).or_insert(0) += 1;
        }
    }

    for index in 0..chars.keys().len() {
        let list = chars.entry(index).or_insert(HashMap::new());
        // Part 1: let max = list.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();
        //print!("{}", max);
        let min = list.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();
        print!("{}", min);        
    }

    println!("");
}
