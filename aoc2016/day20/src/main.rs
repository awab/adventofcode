use std::fs::read_to_string;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut ranges = vec![];
    for line in lines {
        let parts: Vec<_> = line.split('-').collect();
        ranges.push((parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap()));
    }

    ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut last_range_end = 0;
    for range in ranges {
        if range.0 > last_range_end + 1 {
            println!("First non-blacklisted value: {}", last_range_end + 1);
            println!("Next Range: {} {}", range.0, range.1);
            return;
        } else {
            last_range_end = range.1;
        }
    }
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut ranges = vec![];
    for line in lines {
        let parts: Vec<_> = line.split('-').collect();
        ranges.push((parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap()));
    }

    ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut total_ips: u64 = 0;
    let mut last_range_end = 0;
    for range in ranges {
        if range.0 > last_range_end + 1 {
            total_ips += (range.0 - last_range_end - 1) as u64;
        }

        if range.1 > last_range_end {
            last_range_end = range.1;
        }
    }
    
    println!("Total valid IPs: {}", total_ips);
}