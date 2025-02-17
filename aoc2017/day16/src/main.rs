use std::collections::HashMap;
use std::fs::read_to_string;

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn main() {
    let input = "abcdefghijklmnop";
    let filename = "input.txt";
    let mut dance = vec![];
    for ch in input.chars() {
        dance.push(ch);
    }

    let line: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let parts: Vec<_> = line[0].split(",").collect();
    let mut instructions = vec![];
    for part in &parts {
        match part.chars().nth(0).unwrap() {
            's' => {
                let size = part[1..].parse::<usize>().unwrap();
                instructions.push(Instruction::Spin(size));
            },
            'x' => {
                let indexes: Vec<_> = part[1..].split("/").collect();
                let index1 = indexes[0].parse::<usize>().unwrap();
                let index2 = indexes[1].parse::<usize>().unwrap();
                instructions.push(Instruction::Exchange(index1, index2));
            },
            'p' => {
                let char1 = part.chars().nth(1).unwrap();
                let char2 = part.chars().nth(3).unwrap();
                instructions.push(Instruction::Partner(char1, char2));
            },
            _ => panic!("Unknown command {}.", part),
        };
    }

    let mut previous_dance = vec![];
    for _ in 0..100000 {
        for instruction in &instructions {
            match instruction {
                Instruction::Spin(size) => {
                    spin(&mut dance, *size);
                },
                Instruction::Exchange(index1, index2) => {
                    exchange(&mut dance, *index1, *index2);
                },
                Instruction::Partner(char1, char2) => {
                    partner(&mut dance, *char1, *char2);
                },
            };

            if dance == previous_dance {
                // infinite loop
                break;
            }

            previous_dance = dance.clone();
        }
    }

    for ch in dance {
        print!("{}", ch);
    }

    println!("");
}

fn spin(dance: &mut Vec<char>, size: usize) {
    dance.rotate_right(size);
}

fn exchange(dance: &mut Vec<char>, index1: usize, index2: usize) {
    let temp = dance[index1];
    dance[index1] = dance[index2];
    dance[index2] = temp;
}

fn partner(dance: &mut Vec<char>, char1: char, char2: char) {
    let index1 = dance.iter().position(|&x| x == char1).unwrap();
    let index2 = dance.iter().position(|&x| x == char2).unwrap();
    exchange(dance, index1, index2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_works() {
        let mut dance = vec!['a', 'b', 'c', 'd', 'e'];
        spin(&mut dance, 3);
        assert_eq!(dance, vec!['c', 'd', 'e', 'a', 'b']);

        spin(&mut dance, 1);
        assert_eq!(dance, vec!['b', 'c', 'd', 'e', 'a']);

        spin(&mut dance, 5);
        assert_eq!(dance, vec!['b', 'c', 'd', 'e', 'a']);
    }

    #[test]
    fn exchange_works() {
        let mut dance = vec!['a', 'b', 'c', 'd', 'e'];
        exchange(&mut dance, 3, 4);
        assert_eq!(dance, vec!['a', 'b', 'c', 'e', 'd']);

        exchange(&mut dance, 0, 4);
        assert_eq!(dance, vec!['d', 'b', 'c', 'e', 'a']);
    }

    #[test]
    fn partner_works() {
        let mut dance = vec!['a', 'b', 'c', 'd', 'e'];
        partner(&mut dance, 'a', 'e');
        assert_eq!(dance, vec!['e', 'b', 'c', 'd', 'a']);
    }
}