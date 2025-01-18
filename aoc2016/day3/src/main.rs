use std::fs::read_to_string;
/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut valid_triangles = 0;
    for line in &lines {
        let parts: Vec<_> = line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            panic!("Invalid triangle!");
        }

        let length1 = parts[0].parse::<u16>().unwrap();
        let length2 = parts[1].parse::<u16>().unwrap();
        let length3 = parts[2].parse::<u16>().unwrap();

        if length1 + length2 > length3 && length2 + length3 > length1 && length1 + length3 > length2 {
            valid_triangles += 1;
        }
    }

    println!("Total checks: {}", lines.len());
    println!("Total triangles: {}", valid_triangles);
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut valid_triangles = 0;
    let mut iter = lines.iter();
    while let Some(mut current_line) = iter.next() {
        let mut triangle1 = vec![];
        let mut triangle2 = vec![];
        let mut triangle3 = vec![];
        
        for i in 0..=2 {
            let parts: Vec<_> = current_line.trim().split_whitespace().collect();
            if parts.len() != 3 {
                panic!("Invalid triangle!");
            }
    
            let length1 = parts[0].parse::<u16>().unwrap();
            let length2 = parts[1].parse::<u16>().unwrap();
            let length3 = parts[2].parse::<u16>().unwrap();
       
            triangle1.push(length1);
            triangle2.push(length2);
            triangle3.push(length3);

            if i != 2 {
                current_line = iter.next().unwrap();
            }
        }

        if triangle1[0] + triangle1[1] > triangle1[2] && triangle1[1] + triangle1[2] > triangle1[0] && triangle1[0] + triangle1[2] > triangle1[1] {
            valid_triangles += 1;
        }

        if triangle2[0] + triangle2[1] > triangle2[2] && triangle2[1] + triangle2[2] > triangle2[0] && triangle2[0] + triangle2[2] > triangle2[1] {
            valid_triangles += 1;
        }

        if triangle3[0] + triangle3[1] > triangle3[2] && triangle3[1] + triangle3[2] > triangle3[0] && triangle3[0] + triangle3[2] > triangle3[1] {
            valid_triangles += 1;
        }
    }

    println!("Total checks: {}", lines.len());
    println!("Total triangles: {}", valid_triangles);
}