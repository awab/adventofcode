use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let filename = "input.txt";
    let line: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut buckets = vec![];
    let mut cache = vec![];
    let mut steps = 0;
    let parts: Vec<_> = line[0].split_whitespace().collect();
    for part in parts {
        buckets.push(part.parse::<u8>().unwrap());
    }

    let mut found = None;

    loop {
        let biggest = find_biggest_index(&buckets);
        let value = buckets[biggest];
        buckets[biggest] = 0;

        let mut index = biggest + 1;

        // redistribute
        for _ in 0..value {
            if index >= buckets.len() {
                index = 0;
            }

            buckets[index] += 1;
            index += 1;
        }

        steps += 1;

        let concatenation = buckets.clone().into_iter().fold("".to_string(), |acc, x| acc + &x.to_string());
        if cache.contains(&concatenation) {
            if found == None {
                found = Some(concatenation);
                steps = 0;
            } else if Some(concatenation) == found {
                println!("Took {} steps", steps);
                return;    
            }
        } else {
            cache.push(concatenation);
        }
    }
}

fn find_biggest_index(buckets: &Vec<u8>) -> usize {
    let mut result = 0;
    let mut index = 0;
    let mut max: u8 = 0;
    for bucket in buckets {
        if *bucket > max {
            max = *bucket;
            result = index;
        }

        index += 1;
    }

    result
}