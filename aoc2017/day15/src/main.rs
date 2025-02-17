fn main() {
    let mut generator_a: u64 = 289;
    let mut generator_b: u64 = 629;
    let mut score = 0;

    /* Part 1
    for index in 1..40000000 {
        generator_a *= 16807;
        generator_b *= 48271;

        generator_a %= 2147483647;
        generator_b %= 2147483647;

        let generator_a_check = generator_a & 0xFFFF;
        let generator_b_check = generator_b & 0xFFFF;

        if generator_a_check == generator_b_check {
            score += 1;
        }
    }

    println!("Score: {}", score);
    */
    let mut generator_a_picks = vec![];
    let mut generator_b_picks = vec![];
    for index in 1..=40000000 {
        generator_a *= 16807;
        generator_b *= 48271;

        generator_a %= 2147483647;
        generator_b %= 2147483647;

        if generator_a % 4 == 0 {
            generator_a_picks.push(generator_a)
        }

        if generator_b % 8 == 0 {
            generator_b_picks.push(generator_b)
        }
    }

    let mut len = generator_a_picks.len();
    if generator_b_picks.len() < len {
        len = generator_b_picks.len();
    }

    for index in 0..len {
        let generator_a_check = generator_a_picks[index] & 0xFFFF;
        let generator_b_check = generator_b_picks[index] & 0xFFFF;

        if generator_a_check == generator_b_check {
            score += 1;
        }
    }

    println!("Score: {}", score);
}
