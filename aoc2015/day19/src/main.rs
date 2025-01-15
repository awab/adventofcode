use std::collections::HashMap;
use std::fs::read_to_string;

use itertools::Itertools;
use rand::seq::SliceRandom;

/* Day1
fn main() {
    let initial_input = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let parts: Vec<_> = line.split(' ').collect();
        let mut entry = replacements.entry(String::from(parts[0])).or_default();
        entry.push(String::from(parts[2]));
    }

    let mut processed = vec![];
    let mut results = vec![];
    let mut last_char = '\0';
    for index in 0..initial_input.len() {
        // check this char for replacements
        let current_char = initial_input.chars().nth(index).unwrap();
        let entry = replacements.get(&String::from(current_char));
        if entry != None {
            let entry = entry.unwrap();
            for replacement in entry {
                println!("Replacing {} with {}", current_char, replacement);
                let mut result: String = processed.concat();
                result.push_str(&replacement);
                result.push_str(&initial_input[(index + 1)..]);
                results.push(result);
            }
        } else {
            // it could be part of a two-letter key
            let mut two_letter_entry = String::from(last_char);
            two_letter_entry.push(current_char);
            let entry = replacements.get(&two_letter_entry);
            if entry != None {
                let entry = entry.unwrap();
                for replacement in entry {
                    println!("Replacing {} with {}", two_letter_entry, replacement);
                    // we have to remove the last character
                    let mut result: String = processed[0..processed.len() - 1].concat();
                    result.push_str(&replacement);
                    result.push_str(&initial_input[(index + 1)..]);
                    results.push(result);
                }
            }
        }

        processed.push(current_char.to_string());
        last_char = current_char;
    }

    let deduped: Vec<_> = results.iter().unique().collect();
    println!("{}", deduped.len());
}
*/
fn main() {
    let mut initial_input = String::from("CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl");
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut replacements = vec![];
    for line in lines {
        // mapped in reverse from Day 1
        let parts: Vec<_> = line.split(' ').collect();
        replacements.push((String::from(parts[2]), String::from(parts[0])));
    }

    let mut mutation_count = 0;
    while initial_input != "e" {
        // choose a random replacement
        let replacement = replacements.choose(&mut rand::thread_rng()).unwrap();
        if initial_input.contains(&replacement.0) {
            initial_input = initial_input.replacen(&replacement.0, &replacement.1, 1);
            mutation_count += 1;    
        }
    }

    println!("Took {} mutations.", mutation_count);
}