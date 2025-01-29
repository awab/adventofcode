use std::cmp::Reverse;

use bitflags::bitflags;
use itertools::Itertools;
use priority_queue::PriorityQueue;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Types: u8 {
        const None      = 0b0000000;
        const Strontium = 0b0000001;
        const Plutonium = 0b0000010;
        const Thulium   = 0b0000100;
        const Ruthenium = 0b0001000;
        const Curium    = 0b0010000;
        const Elerium   = 0b0100000;
        const Dilithium = 0b1000000;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub move_number: usize,
    pub elevator_index: usize,
    pub elevator_contents: (Types, Types),
    pub floors: Vec<(Types, Types)>,
}

fn main() {
    // the tuple is: (generators, microchips) 
    let mut floors = vec![];
    // Part 1
    //floors.push((Types::Strontium | Types::Plutonium, Types::Strontium | Types::Plutonium));
    //floors.push((Types::Thulium | Types::Ruthenium | Types::Curium, Types::Ruthenium | Types::Curium));
    //floors.push((Types::None, Types::Thulium));
    //floors.push((Types::None, Types::None));

    // Example from text
    //floors.push((Types::None, Types::Strontium | Types::Plutonium));
    //floors.push((Types::Strontium, Types::None));
    //floors.push((Types::Plutonium, Types::None));
    //floors.push((Types::None, Types::None));

    // Part 2
    floors.push((Types::Strontium | Types::Plutonium | Types::Elerium | Types::Dilithium, Types::Strontium | Types::Plutonium | Types::Elerium | Types::Dilithium));
    floors.push((Types::Thulium | Types::Ruthenium | Types::Curium, Types::Ruthenium | Types::Curium));
    floors.push((Types::None, Types::Thulium));
    floors.push((Types::None, Types::None));

    let state = State {
        move_number: 0,
        elevator_index: 0,
        elevator_contents: (Types::None, Types::None),
        floors: floors,
    };

    let mut pq = PriorityQueue::new();
    let mut seen = vec![];

    // we take the first batch of permutations and set them as all equal in the queue
    for permutation in get_permutations(&state.floors, state.elevator_index) {
        let mut new_state = state.clone();
        new_state.move_number = 1;
        new_state.floors[new_state.elevator_index].0 &= !permutation.0;
        new_state.floors[new_state.elevator_index].1 &= !permutation.1;

        new_state.elevator_contents = permutation;
        new_state.elevator_index = 1;

        let priority = new_state.move_number;
        pq.push(new_state, Reverse(priority));
    }

    while pq.len() > 0 {
        let mut state = pq.pop().unwrap().0;

        if !check_floors(&state.floors, state.elevator_index, state.elevator_contents) {
            // invalid setup, we explode and return the max to invalidate the result
            continue;
        }

        // apply the elevator
        state.floors[state.elevator_index].0 |= state.elevator_contents.0;
        state.floors[state.elevator_index].1 |= state.elevator_contents.1;

        if check_is_complete(&state.floors) {
            // since we use a priority queue, the first one to complete should be the lowest number
            println!("Lowest # of moves: {}", state.move_number);
            break;
        }

        // parts are interchangeable
        // find the unique state by counting the number of generators and microchips on each floor
        // since we already checked above that this is a valid state
        let part_counts = get_part_counts(&state.floors);
        if seen.contains(&(part_counts.clone(), state.elevator_index)) {
            //println!("Pruned!");
            continue;
        } else {
            seen.push((part_counts, state.elevator_index));
        }

        // we need to permutate the combinations on the current floor and then iterate up and down for each one
        for permutation in get_permutations(&state.floors, state.elevator_index) {
            // we clone the state, mask out the elevator contents and then shift it in any direction it can go
            let mut new_state = state.clone();
            new_state.move_number += 1;
            new_state.floors[new_state.elevator_index].0 &= !permutation.0;
            new_state.floors[new_state.elevator_index].1 &= !permutation.1;
            
            new_state.elevator_contents = permutation;
            
            // for the middle floors where we can go up or down, we try both
            if state.elevator_index == 0 {
                // up only
                new_state.elevator_index = 1;
                let priority = new_state.move_number;
                pq.push(new_state, Reverse(priority));
            } else if state.elevator_index == 3 {
                if permutation.0 == Types::None || permutation.1 == Types::None {
                    // down - it only makes sense to go down with one object
                    new_state.elevator_index = 2;
                    let priority = new_state.move_number;
                    pq.push(new_state, Reverse(priority));
                }
            } else if state.elevator_index == 1 {
                if permutation.0 == Types::None || permutation.1 == Types::None {
                    // down - it only makes sense to go down with one object
                    new_state.elevator_index = 0;
                    let priority = new_state.move_number;
                    pq.push(new_state.clone(), Reverse(priority));
                }
                
                // up
                let mut new_state = new_state.clone();
                new_state.elevator_index = 2;
                let priority = new_state.move_number;
                pq.push(new_state, Reverse(priority));
            } else if state.elevator_index == 2 {
                if permutation.0 == Types::None || permutation.1 == Types::None {
                    // down - it only makes sense to go down with one object
                    new_state.elevator_index = 1;
                    let priority = new_state.move_number;
                    pq.push(new_state.clone(), Reverse(priority));
                }

                // up
                let mut new_state = new_state.clone();
                new_state.elevator_index = 3;
                let priority = new_state.move_number;
                pq.push(new_state, Reverse(priority));
            }
        }
    }
}

fn check_floors(floors: &Vec<(Types, Types)>, elevator_index: usize, elevator_contents: (Types, Types)) -> bool {
    let mut is_valid = true;
    
    for i in 0..4 {
        let (generators, microchips) = &floors[i];
        let mut generators_with_elevator = *generators;
        let mut microchips_with_elevator = *microchips;
        
        if elevator_index == i {
            // we have to consider the elevator if it's on the same floor
            generators_with_elevator |= elevator_contents.0;
            microchips_with_elevator |= elevator_contents.1;
        }

        if generators_with_elevator != Types::None && microchips_with_elevator != Types::None {
            // we only have to do a check if there are both microchips and generators
            // this checks that all microchips are covered by generators
            is_valid = microchips_with_elevator & generators_with_elevator == microchips_with_elevator;
        }
    }

    is_valid
}

fn check_is_complete(floors: &Vec<(Types, Types)>) -> bool {
    let part_counts = get_part_counts(floors);
    for i in 0..=2 {
        if part_counts[i as usize].0 > 0 || part_counts[i as usize].1 > 0 {
            return false;
        }
    }

    true
}

// returns the various permutations that could make up the elevator of a given floor, they still need to be masked out of the floors themselves
fn get_permutations(floors: &Vec<(Types, Types)>, elevator_index: usize) -> Vec<(Types, Types)> {
    let (generators, microchips) = floors[elevator_index];
    let mut results = vec![];
    // since generators and microchips are the same enum, we encode them as simple strings, make the permutations and then convert them back
    let mut permutations_set = vec![];

    // permutations can be groups of 1 or 2
    if generators & Types::Strontium == Types::Strontium {
        results.push((Types::Strontium, Types::None));
        permutations_set.push("SG");
    }

    if generators & Types::Plutonium == Types::Plutonium {
        results.push((Types::Plutonium, Types::None));
        permutations_set.push("PG");
    }

    if generators & Types::Thulium == Types::Thulium {
        results.push((Types::Thulium, Types::None));
        permutations_set.push("TG");
    }

    if generators & Types::Ruthenium == Types::Ruthenium {
        results.push((Types::Ruthenium, Types::None));
        permutations_set.push("RG");
    }

    if generators & Types::Curium == Types::Curium {
        results.push((Types::Curium, Types::None));
        permutations_set.push("CG");
    }

    if generators & Types::Elerium == Types::Elerium {
        results.push((Types::Elerium, Types::None));
        permutations_set.push("EG");
    }

    if generators & Types::Dilithium == Types::Dilithium {
        results.push((Types::Dilithium, Types::None));
        permutations_set.push("DG");
    }

    if microchips & Types::Strontium == Types::Strontium {
        results.push((Types::None, Types::Strontium));
        permutations_set.push("SM");
    }

    if microchips & Types::Plutonium == Types::Plutonium {
        results.push((Types::None, Types::Plutonium));
        permutations_set.push("PM");
    }

    if microchips & Types::Thulium == Types::Thulium {
        results.push((Types::None, Types::Thulium));
        permutations_set.push("TM");
    }

    if microchips & Types::Ruthenium == Types::Ruthenium {
        results.push((Types::None, Types::Ruthenium));
        permutations_set.push("RM");
    }
    
    if microchips & Types::Curium == Types::Curium {
        results.push((Types::None, Types::Curium));
        permutations_set.push("CM");
    }
    
    if microchips & Types::Elerium == Types::Elerium {
        results.push((Types::None, Types::Elerium));
        permutations_set.push("EM");
    }
    
    if microchips & Types::Dilithium == Types::Dilithium {
        results.push((Types::None, Types::Dilithium));
        permutations_set.push("DM");
    }

    for permutation in permutations_set.iter().combinations(2) {
        let mut generator = Types::None;
        let mut microchip = Types::None;
        match *permutation[0] {
            "SG" => generator |= Types::Strontium,
            "PG" => generator |= Types::Plutonium,
            "TG" => generator |= Types::Thulium,
            "RG" => generator |= Types::Ruthenium,
            "CG" => generator |= Types::Curium,
            "EG" => generator |= Types::Elerium,
            "DG" => generator |= Types::Dilithium,
            "SM" => microchip |= Types::Strontium,
            "PM" => microchip |= Types::Plutonium,
            "TM" => microchip |= Types::Thulium,
            "RM" => microchip |= Types::Ruthenium,
            "CM" => microchip |= Types::Curium,
            "EM" => microchip |= Types::Elerium,
            "DM" => microchip |= Types::Dilithium,
            _ => panic!("Unknown type!"),
        };
        match *permutation[1] {
            "SG" => generator |= Types::Strontium,
            "PG" => generator |= Types::Plutonium,
            "TG" => generator |= Types::Thulium,
            "RG" => generator |= Types::Ruthenium,
            "CG" => generator |= Types::Curium,
            "EG" => generator |= Types::Elerium,
            "DG" => generator |= Types::Dilithium,
            "SM" => microchip |= Types::Strontium,
            "PM" => microchip |= Types::Plutonium,
            "TM" => microchip |= Types::Thulium,
            "RM" => microchip |= Types::Ruthenium,
            "CM" => microchip |= Types::Curium,
            "EM" => microchip |= Types::Elerium,
            "DM" => microchip |= Types::Dilithium,
            _ => panic!("Unknown type!"),
        };

        results.push((generator, microchip));
    }

    results
}

fn get_part_counts(floors: &Vec<(Types, Types)>) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for i in 0..=3 {
        let mut generator_count = 0;
        let mut microchip_count = 0;
        for j in 0..=6 {
            let check: u8 = 2_u8.pow(j);
            if floors[i as usize].0.bits() & check == check {
                generator_count += 1;
            }

            if floors[i as usize].1.bits() & check == check {
                microchip_count += 1;
            }
        }

        results.push((generator_count, microchip_count));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_floors_works() {
        let mut floors = vec![(Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        let mut elevator_index: usize = 0;
        let mut elevator_contents = (Types::None, Types::None);
        let mut result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, true);

        // Same generator and chip, this should be fine
        floors = vec![(Types::Thulium, Types::Thulium), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, true);

        // Same generator and chip with an extra generator, this should be fine
        floors = vec![(Types::Thulium | Types::Curium, Types::Thulium), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, true);

        // No Curium generator, but a Curium chip, this should explode
        floors = vec![(Types::Thulium, Types::Thulium | Types::Curium), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, false);

        // different generator and chip, this should explode
        floors = vec![(Types::Thulium, Types::Curium), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, false);

        // Same generator and chip split across the elevator, this should be fine
        floors = vec![(Types::Thulium, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        elevator_contents = (Types::None, Types::Thulium);
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, true);

        // Different generator and chip split across the elevator, this should explode
        floors = vec![(Types::Thulium, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        elevator_contents = (Types::None, Types::Curium);
        result = check_floors(&floors, elevator_index, elevator_contents);
        assert_eq!(result, false);
    }

    #[test]
    fn check_is_complete_works() {
        let mut floors = vec![(Types::Curium, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        let mut result = check_is_complete(&floors);
        assert_eq!(result, false);

        floors = vec![(Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::Curium, Types::Curium)];
        result = check_is_complete(&floors);
        assert_eq!(result, true);

        floors = vec![(Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::Curium | Types::Plutonium, Types::Curium | Types::Plutonium)];
        result = check_is_complete(&floors);
        assert_eq!(result, true);
    }

    #[test]
    fn test_get_permutations() {
        let mut floors = vec![(Types::Curium, Types::None), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        let mut result = get_permutations(&floors, 0);
        let mut expected = vec![(Types::Curium, Types::None)];
        assert_eq!(result, expected);

        floors = vec![(Types::Curium | Types::Plutonium | Types::Strontium, Types::Ruthenium), (Types::None, Types::None), (Types::None, Types::None), (Types::None, Types::None)];
        result = get_permutations(&floors, 0);
        expected = vec![(Types::Strontium, Types::None), (Types::Plutonium, Types::None), (Types::Curium, Types::None), (Types::None, Types::Ruthenium), (Types::Plutonium | Types::Strontium, Types::None), (Types::Curium | Types::Strontium, Types::None), (Types::Strontium, Types::Ruthenium), (Types::Curium | Types::Plutonium, Types::None), (Types::Plutonium, Types::Ruthenium), (Types::Curium, Types::Ruthenium)];
        assert_eq!(result, expected);
    }
}