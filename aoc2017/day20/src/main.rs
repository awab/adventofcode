use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Particle {
    pub position: (i128, i128, i128),
    pub velocity: (i128, i128, i128),
    pub acceleration: (i128, i128, i128),
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut particles = vec![];
    for line in lines {
        let parts: Vec<_> = line.split(", ").collect();
        let particle = Particle{
            position:       parse_input(parts[0]),
            velocity:       parse_input(parts[1]),
            acceleration:   parse_input(parts[2]),
        };

        particles.push(particle);
    }

    let iterations = 10000;
    for _ in 0..iterations {
        for particle in &mut particles {
            particle.velocity.0 += particle.acceleration.0;
            particle.velocity.1 += particle.acceleration.1;
            particle.velocity.2 += particle.acceleration.2;
            particle.position.0 += particle.velocity.0;
            particle.position.1 += particle.velocity.1;
            particle.position.2 += particle.velocity.2;
        }

        let mut collisions = HashMap::new();
        for particle in &particles {
            let mut entry = collisions.entry(particle.position.clone()).or_insert(0);
            *entry += 1;
        }

        for key in collisions.keys() {
            let collision_points = collisions.get(&key).unwrap();
            if *collision_points > 1 {
                let len = particles.len();
                for index in (0..particles.len()).rev() {
                    if particles[index].position.0 == key.0 && particles[index].position.1 == key.1 && particles[index].position.2 == key.2 {
                        particles.remove(index);
                    }
                }
                println!("Purging to: {}", particles.len());
            }
        }
    }

    // Part 2
    println!("{} particles remaining", particles.len());

    /* Part 1
    let mut distances = vec![];
    for particle in particles {
        let distance = get_distance_from_origin(&particle);
        distances.push(distance);
    }

    let minimum_particle = distances
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap();

    println!("Particle {} is closest at {} away", minimum_particle, distances[minimum_particle]);
    */
}

fn parse_input(input: &str) -> (i128, i128, i128) {
    // form x=<n,-n, n>
    let numbers: Vec<_> = input[3..input.len() - 1].split(",").collect();
    (numbers[0].trim().parse::<i128>().unwrap(), numbers[1].trim().parse::<i128>().unwrap(), numbers[2].trim().parse::<i128>().unwrap())
}

fn get_distance_from_origin(particle: &Particle) -> i128 {
    particle.position.0.abs() + particle.position.1.abs() + particle.position.2.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_works() {
        let result = parse_input("p=<1,-2,0>");
        assert_eq!(result, (1, -2, 0));
    }

    #[test]
    fn get_distance_from_origin_works() {
        let mut particle = Particle {
            position: (0, 0, 0),
            velocity: (0, 0, 0),
            acceleration: (0, 0, 0),
        };

        let mut result = get_distance_from_origin(&particle);
        assert_eq!(result, 0);

        // test a move in any given direction
        particle.position.0 = 2;
        result = get_distance_from_origin(&particle);
        assert_eq!(result, 2);

        particle.position.0 = 0;
        particle.position.1 = -3;
        result = get_distance_from_origin(&particle);
        assert_eq!(result, 3);

        particle.position.1 = 0;
        particle.position.2 = 4;
        result = get_distance_from_origin(&particle);
        assert_eq!(result, 4);

        // test three dimensional move
        particle.position.0 = -100;
        particle.position.1 = 200;
        particle.position.2 = -300;
        result = get_distance_from_origin(&particle);
        assert_eq!(result, 600);
    }
}