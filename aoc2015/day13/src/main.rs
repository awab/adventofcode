use std::collections::HashMap;
use std::fs::read_to_string;
use std::rc::Rc;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"([^\s]+)\swould\s(gain|lose)\s(\d+)\shappiness\sunits\sby\ssitting\snext\sto\s([^.]+).").unwrap();
    
    let mut people: Vec<Rc<str>> = vec![];
    let mut relationships: HashMap<Rc<str>, HashMap<Rc<str>, i32>> = HashMap::new();
    for line in lines {
        for (_, [first_person, action, units, second_person]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            if !people.contains(&first_person.into()) {
                people.push(first_person.into());
            }
            if !people.contains(&second_person.into()) {
                people.push(second_person.into());
            }

            let person_map = relationships.entry(first_person.into()).or_default();
            let value = if action == "gain" { 1 } else { -1 } * units.parse::<i32>().unwrap();
            person_map.insert(second_person.into(), value);
            person_map.insert(String::from("Adam").into(), 0);
        }
    }

    let mut happy_permutation = i32::MIN;
    for permutation in people.iter().permutations(people.len()).unique() {
        let mut permutation_total = 0;
        for i in 0..permutation.len() {
            if i == 0 {
                // leftmost index, get the person on the other end
                let relationship_map = relationships.get(permutation[0]).unwrap();
                let left_value = relationship_map.get(permutation[permutation.len() - 1]).unwrap();
                let right_value = relationship_map.get(permutation[1]).unwrap();
                permutation_total += left_value;
                permutation_total += right_value;
            } else if i == permutation.len() - 1 {
                // rightmost index, get the person on the other end
                let relationship_map = relationships.get(permutation[permutation.len() - 1]).unwrap();
                let left_value = relationship_map.get(permutation[i - 1]).unwrap();
                let right_value = relationship_map.get(permutation[0]).unwrap();
                permutation_total += left_value;
                permutation_total += right_value;
            } else {
                let relationship_map = relationships.get(permutation[i]).unwrap();
                let left_value = relationship_map.get(permutation[i - 1]).unwrap();
                let right_value = relationship_map.get(permutation[i + 1]).unwrap();
                permutation_total += left_value;
                permutation_total += right_value;
            }
        }
        if permutation_total > happy_permutation {
            happy_permutation = permutation_total;
        }
    }

    println!("Best permutation total: {}", happy_permutation);
}
