use std::collections::HashMap;
use std::fs::read_to_string;

struct Sue {
    children: i16,
    cats: i16,
    samoyeds: i16,
    pomeranians: i16,
    akitas: i16,
    vizslas: i16,
    goldfish: i16,
    trees: i16,
    cars: i16,
    perfumes: i16,
}

/* Day1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();

    let mut matching_sue = None;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        // remove the :
        let sue_number = parts[1][0..parts[1].len() - 1].parse::<i16>().unwrap();
        
        // the rest should be in pairs of results
        if parts.len() % 2 != 0 {
            panic!("Invalid parts length!");
        }

        let mut current_sue = Sue {
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        };

        for index in (2..parts.len()).step_by(2) {
            let index = index as usize;
            let mut key = parts[index];
            if parts[index].chars().last().unwrap() == ':' {
                key = &parts[index][0..parts[index].len() - 1];
            }
            let mut value = parts[index + 1];
            if parts[index + 1].chars().last().unwrap() == ',' {
                value = &parts[index + 1][0..parts[index + 1].len() - 1];
            }
            let value = value.parse::<i16>().unwrap();
            match key {
                "children" => current_sue.children = value,
                "cats" => current_sue.cats = value,
                "samoyeds" => current_sue.samoyeds = value,
                "pomeranians" => current_sue.pomeranians = value,
                "akitas" => current_sue.akitas = value,
                "vizslas" => current_sue.vizslas = value,
                "goldfish" => current_sue.goldfish = value,
                "trees" => current_sue.trees = value,
                "cars" => current_sue.cars = value,
                "perfumes" => current_sue.perfumes = value,
                _ => panic!("Unexpected key: {}", key),
            };
        }

        if (current_sue.children == 3 || current_sue.children == -1) &&
           (current_sue.cats == 7 || current_sue.cats == -1) &&
           (current_sue.samoyeds == 2 || current_sue.samoyeds == -1) &&
           (current_sue.pomeranians == 3 || current_sue.pomeranians == -1) &&
           (current_sue.akitas == 0 || current_sue.akitas == -1) &&
           (current_sue.vizslas == 0 || current_sue.vizslas == -1) &&
           (current_sue.goldfish == 5 || current_sue.goldfish == -1) &&
           (current_sue.trees == 3 || current_sue.trees == -1) &&
           (current_sue.cars == 2 || current_sue.cars == -1) &&
           (current_sue.perfumes == 1 || current_sue.perfumes == -1) {
            matching_sue = Some(sue_number);
        }
    }     

    if matching_sue != None {
        println!("Matching Sue: {}", matching_sue.unwrap());
    }
}
*/


fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();

    let mut matching_sue = None;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        // remove the :
        let sue_number = parts[1][0..parts[1].len() - 1].parse::<i16>().unwrap();
        
        // the rest should be in pairs of results
        if parts.len() % 2 != 0 {
            panic!("Invalid parts length!");
        }

        let mut current_sue = Sue {
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1,
        };

        for index in (2..parts.len()).step_by(2) {
            let index = index as usize;
            let mut key = parts[index];
            if parts[index].chars().last().unwrap() == ':' {
                key = &parts[index][0..parts[index].len() - 1];
            }
            let mut value = parts[index + 1];
            if parts[index + 1].chars().last().unwrap() == ',' {
                value = &parts[index + 1][0..parts[index + 1].len() - 1];
            }
            let value = value.parse::<i16>().unwrap();
            match key {
                "children" => current_sue.children = value,
                "cats" => current_sue.cats = value,
                "samoyeds" => current_sue.samoyeds = value,
                "pomeranians" => current_sue.pomeranians = value,
                "akitas" => current_sue.akitas = value,
                "vizslas" => current_sue.vizslas = value,
                "goldfish" => current_sue.goldfish = value,
                "trees" => current_sue.trees = value,
                "cars" => current_sue.cars = value,
                "perfumes" => current_sue.perfumes = value,
                _ => panic!("Unexpected key: {}", key),
            };
        }

        if (current_sue.children == 3 || current_sue.children == -1) &&
           (current_sue.cats > 7 || current_sue.cats == -1) &&
           (current_sue.samoyeds == 2 || current_sue.samoyeds == -1) &&
           (current_sue.pomeranians < 3 || current_sue.pomeranians == -1) &&
           (current_sue.akitas == 0 || current_sue.akitas == -1) &&
           (current_sue.vizslas == 0 || current_sue.vizslas == -1) &&
           (current_sue.goldfish < 5 || current_sue.goldfish == -1) &&
           (current_sue.trees > 3 || current_sue.trees == -1) &&
           (current_sue.cars == 2 || current_sue.cars == -1) &&
           (current_sue.perfumes == 1 || current_sue.perfumes == -1) {
            matching_sue = Some(sue_number);
        }
    }

    if matching_sue != None {
        println!("Matching Sue: {}", matching_sue.unwrap());
    }
}