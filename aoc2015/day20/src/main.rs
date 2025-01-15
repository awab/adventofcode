use factor::factor_include::factor_include;

/* Day 1
fn main() {
    let result = 3310000;
    let mut current_result = 0;
    let mut current_house = 0;
    while result > current_result {
        current_house += 1;
        let factors = factor_include(current_house);
        current_result = 0;
        for factor in factors {
            current_result += factor;
        }
    }

    println!("House #{}", current_house);
}
*/

fn main() {
    let result = 33100000;
    let mut current_result = 0;
    let mut current_house = 0;
    while result > current_result {
        current_house += 1;
        let factors = factor_include(current_house);
        current_result = 0;

        // we loop through half of the factors
        for index in 0..=(factors.len() / 2) {
            // we crawl from the front and back and meet in the middle
            let elf1 = factors[index];
            let elf2 = factors[factors.len() - index - 1];

            if elf1 <= 50 {
                // elf1 is the number of times elf2 has been to this house
                current_result += elf2 * 11;
            }

            if elf2 <= 50 {
                current_result += elf1 * 11;
            }
        }
    }

    println!("House #{}", current_house);
}