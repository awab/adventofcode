use std::collections::HashMap;
use std::fs::read_to_string;

use regex::Regex;

struct Reindeer {
    name: String,
    speed: u8,
    fly_time: u8,
    rest_time: u8,
    is_resting: bool,
    current_action_time: u8,
    total_distance: u32,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"([^\s]+)\scan\sfly\s(\d+)\skm/s\sfor\s(\d+)\sseconds,\sbut\sthen\smust\srest\sfor\s(\d+)\sseconds.").unwrap();
    
    let mut all_reindeer = vec![];
    let mut scores: HashMap<String, u16> = HashMap::new();
    for line in lines {
        for (_, [reindeer, speed, fly_time, rest_time]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let current_reindeer = Reindeer {
                name: reindeer.to_string(),
                speed: speed.parse::<u8>().unwrap(),
                fly_time: fly_time.parse::<u8>().unwrap(),
                rest_time: rest_time.parse::<u8>().unwrap(),
                is_resting: true,
                current_action_time: 0,
                total_distance: 0,
            };

            all_reindeer.push(current_reindeer);
        }
    }

    let mut flight_time = 2503;
    while flight_time > 0 {
        for reindeer in all_reindeer.iter_mut() {
            if reindeer.current_action_time == 0 {
                reindeer.is_resting = !reindeer.is_resting;
                if reindeer.is_resting {
                    reindeer.current_action_time = reindeer.rest_time;
                } else {
                    reindeer.current_action_time = reindeer.fly_time;
                }
            }

            if !reindeer.is_resting {
                reindeer.total_distance += reindeer.speed as u32;
            }
            reindeer.current_action_time -= 1;
        }

        let mut winning_reindeer = vec![];
        let mut max_distance = 0;
        for reindeer in all_reindeer.iter() {
            if reindeer.total_distance > max_distance {
                winning_reindeer.clear();
                max_distance = reindeer.total_distance;
                winning_reindeer.push(reindeer.name.clone());
            } else if reindeer.total_distance == max_distance {
                // we can have ties
                winning_reindeer.push(reindeer.name.clone());
            }
        }

        for reindeer in winning_reindeer {
            *scores.entry(reindeer).or_default() += 1;
        }

        flight_time -= 1;
    }

    for reindeer in all_reindeer.iter() {
        let score = match scores.get(&reindeer.name) {
            Some(score) => score,
            None => &0,
        };
        
        println!("{} received {} points", reindeer.name, score);
    }
}
