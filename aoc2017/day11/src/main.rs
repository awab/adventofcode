use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let line: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    
    /* Hexagon "Cube" coordinate system:
    https://i.sstatic.net/DWVQk.png
    */
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    let directions: Vec<_> = line[0].split(",").collect();
    let mut max = 0;
    for direction in directions {
        if direction == "nw" {
            x -= 1;
            y += 1;
        } else if direction == "n" {
            y += 1;
            z -= 1;
        } else if direction == "ne" {
            x += 1;
            z -= 1;
        } else if direction == "sw" {
            x -= 1;
            z += 1;
        } else if direction == "s" {
            y -= 1;
            z += 1;
        } else if direction == "se" {
            x += 1;
            y -= 1;
        } 

        let distance = (x.abs() + y.abs() + z.abs()) / 2;
        if distance > max {
            max = distance;
        }
    }

    let distance = (x.abs() + y.abs() + z.abs()) / 2;
    println!("Distance: {}", distance); // Part 1
    println!("Max Distance: {}", max); // Part 2
}
