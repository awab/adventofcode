use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}

fn main() {
    let filename = "input.txt";
    let mut data = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut data));
    
    let mut coordinates = HashMap::new();
    let mut santa_current_coordinates = Point { x: 0, y: 0 };
    let mut robo_current_coordinates = Point { x: 0, y: 0 };
    coordinates.insert(santa_current_coordinates.clone(), 1); // the starting location counts
    let mut index = 0;
    for direction in data {
        if index % 2 == 0 {
            match direction {
                b'>' => santa_current_coordinates.x += 1,
                b'<' => santa_current_coordinates.x -= 1,
                b'^' => santa_current_coordinates.y += 1,
                b'v' => santa_current_coordinates.y -= 1,
                _ => { },
            };
    
            if !coordinates.contains_key(&santa_current_coordinates) {
                coordinates.insert(santa_current_coordinates, 1);
            } else {
                let existing = coordinates.entry(santa_current_coordinates).or_insert(0);
                *existing += 1;
            }    
        } else {
            match direction {
                b'>' => robo_current_coordinates.x += 1,
                b'<' => robo_current_coordinates.x -= 1,
                b'^' => robo_current_coordinates.y += 1,
                b'v' => robo_current_coordinates.y -= 1,
                _ => { },
            };
    
            if !coordinates.contains_key(&robo_current_coordinates) {
                coordinates.insert(robo_current_coordinates, 1);
            } else {
                let existing = coordinates.entry(robo_current_coordinates).or_insert(0);
                *existing += 1;
            }    
        }

        index += 1;
    }

    println!("Total houses: {}", coordinates.keys().len());
}
