use std::fs::read_to_string;

/* Part 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut checks = 0;
    let mut valid_ips = 0;

    for line in lines {
        checks += 1;
        if check_valid(line) {
            valid_ips += 1;
        }
    }

    println!("Checks: {}", checks);
    println!("Valid ips: {}", valid_ips);
}

fn check_valid(input: String) -> bool {
    let mut chars = input.chars();
    let mut char1 = chars.next().unwrap();
    let mut char2 = chars.next().unwrap();
    let mut char3 = chars.next().unwrap();
    let mut is_check = false;
    let mut found_abba = false;

    // we queue up the first three and then iterate through every batch of four
    loop {
        let char4 = chars.next();
        if char4 == None {
            break;
        }

        let char4 = char4.unwrap();
        if char4 == '[' {
            is_check = true;

            // reset the checks before a [] sequence
            char1 = '\0';
            char2 = '\0';
            char3 = '\0';

            continue;
        } else if char4 == ']' {
            is_check = false;
            
            // reset the checks after a [] sequence
            char1 = '\0';
            char2 = '\0';
            char3 = '\0';
            
            continue;
        }

        // look for abba patterns, but not aaaa
        if char1 == char4 && char2 == char3 && char1 != char2 {
            if is_check {
                return false;
            }

            found_abba = true;
        }

        char1 = char2;
        char2 = char3;
        char3 = char4;
    }

    found_abba
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_works() {
        let mut result = check_valid(String::from("abba"));
        assert_eq!(result, true);

        // check if it works when in an odd position
        result = check_valid(String::from("xxxxxabbaxxxxxxxx"));
        assert_eq!(result, true);

        // checks from examples
        result = check_valid(String::from("abba[mnop]qrst"));
        assert_eq!(result, true);

        result = check_valid(String::from("abcd[bddb]xyyx"));
        assert_eq!(result, false);

        result = check_valid(String::from("aaaa[qwer]tyui"));
        assert_eq!(result, false);

        result = check_valid(String::from("ioxxoj[asdfgh]zxcvbn"));
        assert_eq!(result, true);

        // two checks
        result = check_valid(String::from("ioxxoj[asdfgh]zxcvbn[oxxo]test"));
        assert_eq!(result, false);

        result = check_valid(String::from("oxxo[asdfgh]zxcvbn[abcde]oxxo"));
        assert_eq!(result, true);

        result = check_valid(String::from("ij[asdfgh]zxcoxxovbn[abcd]test"));
        assert_eq!(result, true);

        result = check_valid(String::from("ij[asdfgh]zxcvbn[abcde]oxxo"));
        assert_eq!(result, true);
        
        result = check_valid(String::from("oxxo[asdfgh]zxcvbn[abcde]test"));
        assert_eq!(result, true);

        result = check_valid(String::from("fxhdruviojyidmpkxsm[dlbaklivbcycxgcz]zeaqtsnkqhvsbfsquey[yespxpiododicfl]lsjpyjbyqhhvvmaam"));
        assert_eq!(result, true);
    }
}
*/



fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut checks = 0;
    let mut valid_ips = 0;

    for line in lines {
        checks += 1;
        if check_valid(line) {
            valid_ips += 1;
        }
    }

    println!("Checks: {}", checks);
    println!("Valid ips: {}", valid_ips);
}

fn check_valid(input: String) -> bool {
    let mut chars = input.chars();
    let mut char1 = chars.next().unwrap();
    let mut char2 = chars.next().unwrap();
    let mut is_check = false;
    let mut aba_results = vec![];
    let mut bab_results = vec![];

    // we queue up the first two and then iterate through every batch of three
    loop {
        let char3 = chars.next();
        if char3 == None {
            break;
        }

        let char3 = char3.unwrap();
        if char3 == '[' {
            is_check = true;

            // reset the checks before a [] sequence
            char1 = '\0';
            char2 = '\0';

            continue;
        } else if char3 == ']' {
            is_check = false;
            
            // reset the checks after a [] sequence
            char1 = '\0';
            char2 = '\0';
            
            continue;
        }

        // look for aba patterns, but not aaaa
        if char1 == char3 && char1 != char2 {
            let mut result = char1.to_string();
            result.push(char2);
            result.push(char3);

            if !is_check {
                aba_results.push(result);
            } else {
                bab_results.push(result);
            }
        }

        char1 = char2;
        char2 = char3;
    }

    for aba_result in aba_results {
        //reverse it to get our search
        let check = aba_result[1..=2].to_string() + &aba_result[1..=1];
        if bab_results.contains(&check) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_valid_works() {
        let mut result = check_valid(String::from("aba[bab]xyz"));
        assert_eq!(result, true);

        result = check_valid(String::from("xyx[xyx]xyx"));
        assert_eq!(result, false);

        result = check_valid(String::from("aaa[kek]eke"));
        assert_eq!(result, true);

        result = check_valid(String::from("zazbz[bzb]cdb"));
        assert_eq!(result, true);
    }
}