use std::fs::File;
use std::io::Read;


fn main() {
    let filename = "input.txt";
    let mut data = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut data));
    
    /* Part 1
    let mut last = '\0';
    let mut count = 1;
    let mut total = 0;

    for byte in &data {
        let ch = *byte as char;
        if last == ch {
            total += last.to_digit(10).unwrap();
        }

        last = ch;
    }

    if data[0] == data[data.len() - 1] {
        total += (data[0] as char).to_digit(10).unwrap();
    }

    println!("Total: {}", total);
    */

    let mut total = 0;
    let len = data.len() / 2;

    for index in 0..len {
        if data[index] == data[index + len] {
            total += (data[index] as char).to_digit(10).unwrap() * 2;
        }
    }

    println!("Total: {}", total);
}
