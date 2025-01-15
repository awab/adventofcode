fn main() {
    let mut input = String::from("1113222113");
    for i in 0..50 {
        let mut encoding = vec![];
        let mut last_char = '\0';
        let mut current_run: u32 = 0;

        let mut index = 0;
        for ch in input.chars() {
            if ch == last_char {
                current_run += 1;
            } else {
                if last_char != '\0' {
                    encoding.push((last_char, current_run));
                }

                last_char = ch;
                current_run = 1;
            }

            index += 1;

            if index == input.len() {
                // make sure the last character is added
                encoding.push((last_char, current_run));
            } 
        }

        input = String::from("");
        for (ch, count) in encoding {
            input = vec![input, count.to_string(), ch.to_string()].concat();
        }
    }

    println!("Final Output Length: {}", input.len());
}
