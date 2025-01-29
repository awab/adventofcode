use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

#[derive(Clone)]
enum Target {
    Output(u16),
    Bot(u16),
}

#[derive(Clone)]
struct Bot {
    pub chips: Vec<u16>,
    pub low_target: Option<Target>,
    pub high_target: Option<Target>,
}


impl Bot {
    fn new() -> Self {
        Bot {
            chips: vec![], 
            low_target: None, 
            high_target: None,    
        }
    }

    fn send_value(&mut self, value: u16) {
        if !self.chips.contains(&value) {
            self.chips.push(value);
        }
    }

    fn get_low(&self) -> u16 {
        *self.chips.iter().min().unwrap()
    }

    fn get_high(&self) -> u16 {
        *self.chips.iter().max().unwrap()
    }
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut bots = HashMap::new();
    let mut signals = VecDeque::new();

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        if line.starts_with("value") {
            // direct value store
            let key = parts[5].parse::<u16>().unwrap();
            let mut bot = bots.entry(key).or_insert(Bot::new());
            bot.send_value(parts[1].parse::<u16>().unwrap());
            signals.push_back(key);
        } else {
            // we essentially build a linked list from the values
            let key = parts[1].parse::<u16>().unwrap();
            let mut bot = bots.entry(key).or_insert(Bot::new());

            let low_value = parts[6].parse::<u16>().unwrap();
            bot.low_target = if parts[5] == "bot" {
                Some(Target::Bot(low_value))
            } else {
                Some(Target::Output(low_value))
            };

            let high_value = parts[11].parse::<u16>().unwrap();
            let high_target = if parts[10] == "bot" {
                Some(Target::Bot(high_value))
            } else {
                Some(Target::Output(high_value))
            };

            bot.high_target = high_target;
        }
    }

    let mut outputs = HashMap::new();

    while signals.len() > 0 {
        let signal = signals.pop_front().unwrap();
        let bot = bots.get_mut(&signal).unwrap().clone();
        if bot.chips.len() == 2 {
            /* Part1
            if bot.get_low() == 17 && bot.get_high() == 61 {
                println!("Comparison found: {}", signal);
            }
            */

            match bot.low_target {
                Some(Target::Bot(target_bot)) => {
                    let target = bots.get_mut(&target_bot).unwrap();
                    target.send_value(bot.get_low());
                    signals.push_back(target_bot)
                },
                Some(Target::Output(output)) => {
                    outputs.insert(output, bot.get_low());
                }
                _ => { },
            };

            match bot.high_target {
                Some(Target::Bot(target_bot)) => {
                    let target = bots.get_mut(&target_bot).unwrap();
                    target.send_value(bot.get_high());
                    signals.push_back(target_bot)
                },
                Some(Target::Output(output)) => {
                    outputs.insert(output, bot.get_high());
                }
                _ => { },
            };
        } else {
            // we requeue it to check later
            signals.push_back(signal);
        }
    }

    println!("{}", bots.len());
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_bot_min_and_max_work() {
        let mut bot = Bot {
            chips: vec![],
        };
        
        bot.send_value(10);
        bot.send_value(20);

        assert_eq!(bot.get_low(), 10);
        assert_eq!(bot.get_high(), 20);
    }
}