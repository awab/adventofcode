use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    // (0, 0) is the upper left corner
    let mut grid = HashMap::new();
    let mut no_overlaps = 0;

    let mut all_coordinates = vec![];
    let mut all_dimensions = vec![];
    
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let coordinates_str = &parts[2][0..parts[2].len() - 1]; // we remove the : at the end
        let coordinates_parts: Vec<_> = coordinates_str.split(",").collect();
        let coordinates = (coordinates_parts[0].parse::<u16>().unwrap(), coordinates_parts[1].parse::<u16>().unwrap());
        let dimensions_parts: Vec<_> = parts[3].split("x").collect();
        let dimensions = (dimensions_parts[0].parse::<u16>().unwrap(), dimensions_parts[1].parse::<u16>().unwrap());
        let mut overlaps = false;
        for y in coordinates.1..(coordinates.1 + dimensions.1) {
            for x in coordinates.0..(coordinates.0 + dimensions.0) {
                *grid.entry((x, y)).or_insert(0) += 1;    
            }
        }

        all_coordinates.push(coordinates);
        all_dimensions.push(dimensions);
    }

    let mut overlap_count = 0;
    for key in grid.keys() {
        let count = grid.get(key).unwrap();
        if *count > 1 {
            overlap_count += 1;
        }
    }

    // we just loop through the squares again looking for the no overlaps
    let mut index = 0;
    for coordinates in all_coordinates {
        let mut overlaps = false;
        let dimensions = all_dimensions[index];
        for y in coordinates.1..(coordinates.1 + dimensions.1) {
            for x in coordinates.0..(coordinates.0 + dimensions.0) {
                let value = grid.get(&(x, y)).unwrap();
                if *value > 1{
                    overlaps = true;
                    break;
                }    
            }

            if overlaps {
                break;
            }
        }

        if !overlaps {
            no_overlaps = index + 1;
        }

        index += 1;
    }

    println!("Overlap count: {}", overlap_count); // Part 1
    println!("No overlaps: {}", no_overlaps); // Part 2
}
