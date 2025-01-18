use std::fs::read_to_string;

#[derive(PartialEq)]
enum Orientation {
    None,
    Up,
    Right,
    Down,
    Left,
}

/* Day1
fn main() {
    let filename = "input.txt";
    let input: String = read_to_string(filename).unwrap();
    let parts: Vec<_> = input.split(", ").collect();

    let mut orientation = Orientation::None;
    let mut x_coordinate = 0;
    let mut y_coordinate = 0;

    for part in parts {
        let direction = part.chars().next().unwrap();
        let amount = part[1..part.len()].trim().parse::<i16>().unwrap();

        orientation = change_orientation(orientation, direction);
        match orientation {
            Orientation::None => panic!("Should never happen!"),
            Orientation::Up => {
                y_coordinate += amount;
            },
            Orientation::Right => {
                x_coordinate += amount;
            },
            Orientation::Down => {
                y_coordinate -= amount;
            },
            Orientation::Left => {
                x_coordinate -= amount;
            },
        };
    }

    println!("Total direct movement: {}", (x_coordinate.abs() + y_coordinate.abs()));
}
*/

fn main() {
    let filename = "input.txt";
    let input: String = read_to_string(filename).unwrap();
    let parts: Vec<_> = input.split(", ").collect();
    
    let mut orientation = Orientation::None;
    let mut x_coordinate: i32 = 0;
    let mut y_coordinate: i32 = 0;
    let mut results = vec![];

    for part in parts {
        let direction = part.chars().next().unwrap();
        let amount = part[1..part.len()].trim().parse::<i16>().unwrap();

        orientation = change_orientation(orientation, direction);
        for i in 1..=amount {
            match orientation {
                Orientation::None => panic!("Should never happen!"),
                Orientation::Up => {
                    y_coordinate += 1;
                },
                Orientation::Right => {
                    x_coordinate += 1;
                },
                Orientation::Down => {
                    y_coordinate -= 1;
                },
                Orientation::Left => {
                    x_coordinate -= 1;
                },
            };

            if results.contains(&(x_coordinate, y_coordinate)) {
                println!("First overlap is: {} blocks away", (x_coordinate.abs() + y_coordinate.abs()));
                return;
            } else {
                results.push((x_coordinate, y_coordinate));
            }
        }
    }

    println!("No overlaps found!");
}

fn change_orientation(orientation: Orientation, direction: char) -> Orientation {
    match orientation {
        Orientation::None => {
            if direction == 'R' {
                Orientation::Right
            } else {
                Orientation::Left
            }
        },
        Orientation::Up => {
            if direction == 'R' {
                Orientation::Right
            } else {
                Orientation::Left
            }
        },
        Orientation::Right => {
            if direction == 'R' {
                Orientation::Down
            } else {
                Orientation::Up
            }
        },
        Orientation::Down => {
            if direction == 'R' {
                Orientation::Left
            } else {
                Orientation::Right
            }
        },
        Orientation::Left => {
            if direction == 'R' {
                Orientation::Up
            } else {
                Orientation::Down
            }
        },
    }
}