use std::fs::File;
use std::io::Read;


fn main() {
    let filename = "input.txt";
    let mut data = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut data));
    
    let mut first_basement_index = -1;
    let mut index = 1;
    let mut floor = 0;
    for direction in data {
        match direction {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => { },
        };

        if first_basement_index == -1 && floor == -1 {
            first_basement_index = index;
        }

        index += 1;
    }

    println!("Floor: {}", floor);
    println!("First Basement Entry: {}", first_basement_index);
}
