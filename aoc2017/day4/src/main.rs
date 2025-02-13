use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut valid = 0;
    'linecheck: for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut pieces = vec![];
        for part in parts {
            let mut chars: Vec<char> = part.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));

            let str_conversion = chars.into_iter().collect::<Vec<_>>();

            if pieces.contains(&str_conversion) {
                continue 'linecheck;
            }

            pieces.push(str_conversion);
        }

        valid += 1;
    }

    println!("Valid passwords: {}", valid);
}
