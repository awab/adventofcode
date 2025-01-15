use md5;

fn main() {
    let prefix = "yzbqklnj".to_string();
    let mut answer: i64 = 1;
    loop {
        answer += 1;
        let digest = md5::compute(prefix.clone() + &answer.to_string());
        
        if format!("{:x}", digest).starts_with("000000") {
            println!("Answer found: {}", answer);
            return;
        }
    }
}
