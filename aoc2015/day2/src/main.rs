use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut total_size = 0;
    let mut total_ribbon_length = 0;
    for line in lines {
        let mut parts = line.split("x");
        let length = parts.next().expect("Shouldn't happen.").parse::<i32>().unwrap();
        let width = parts.next().expect("Shouldn't happen.").parse::<i32>().unwrap();
        let height = parts.next().expect("Shouldn't happen.").parse::<i32>().unwrap();

        let dimension1 = length * width;
        let dimension2 = width * height;
        let dimension3 = height * length;

        let mut smallest_length = dimension1;
        if dimension2 < smallest_length {
            smallest_length = dimension2;
        }
        if dimension3 < smallest_length {
            smallest_length = dimension3;
        }

        let required_area = 
            (2 * dimension1) +
            (2 * dimension2) +
            (2 * dimension3) +
            smallest_length;

        total_size += required_area;


        let perimeter1 = (2 * length) + (2 * width);
        let perimeter2 = (2 * width) + (2 * height);
        let perimeter3 = (2 * length) + (2 * height);
        let mut smallest_perimeter = perimeter1;
        if perimeter2 < smallest_perimeter {
            smallest_perimeter = perimeter2;
        }
        if perimeter3 < smallest_perimeter {
            smallest_perimeter = perimeter3;
        }

        let ribbon_length = smallest_perimeter + (length * width * height);
        total_ribbon_length += ribbon_length;
    }

    println!("Total required sq ft: {}", total_size);
    println!("Total required ribbon ft: {}", total_ribbon_length);
}