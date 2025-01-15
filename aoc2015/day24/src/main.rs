use std::fs::read_to_string;

use itertools::Itertools;

/* Day 1
fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut packages = vec![];
    for line in lines {
        packages.push(line.parse::<u64>().unwrap());
    }

    let final_weight = packages.iter().sum::<u64>() / 3;
    
    // it can't have more than 1/3 of the packages or it won't have the least
    let cap = packages.len() / 3;
    let mut current_winning_package_count = u64::MAX;
    let mut current_winning_quantum_entanglement = u64::MAX;

    for first_package_count in 2..cap {
        if first_package_count > current_winning_package_count as usize {
            // it's impossible to find a better solution
            break;
        }

        for first_combination in packages.iter().combinations(first_package_count) {
            let first_size: u64 = first_combination.clone().into_iter().sum();
            if first_size != final_weight {
                continue;
            }

            let remaining_packages_count = packages.len() - first_package_count;
            for second_package_count in (first_package_count + 1)..(remaining_packages_count - 2) {
                let mut remaining_packages = packages.clone();
                remaining_packages.retain(|x| !first_combination.contains(&x));
                for second_combination in remaining_packages.iter().combinations(second_package_count) {
                    let second_size: u64 = second_combination.clone().into_iter().sum();
                    if first_size == second_size {
                        // if not, there's no need to continue
                        let mut final_remaining_packages = packages.clone();
                        final_remaining_packages.retain(|x| !first_combination.contains(&x) && !second_combination.contains(&x));
                        if final_remaining_packages.len() <= first_package_count {
                            continue;
                        }

                        let third_size: u64 = final_remaining_packages.clone().into_iter().sum();   
                        if second_size == third_size {
                            if first_package_count <= current_winning_package_count {
                                let current_quantum_entanglement = first_combination.clone().into_iter().product();
                                if current_quantum_entanglement < current_winning_quantum_entanglement {
                                    current_winning_package_count = first_package_count as u64;
                                    current_winning_quantum_entanglement = current_quantum_entanglement;
                                    println!("Found winner. Size: {}, QE: {}", current_winning_package_count, current_winning_quantum_entanglement)
                                }
                            }
                        }
                    }   
                }
            }
        }        
    }

    println!("Lowest balanced quantum entanglement: {}", current_winning_quantum_entanglement);
}
*/

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut packages = vec![];
    for line in lines {
        packages.push(line.parse::<u64>().unwrap());
    }

    let final_weight = packages.iter().sum::<u64>() / 4;
    
    // it can't have more than 1/4 of the packages or it won't have the least
    let cap = packages.len() / 4;
    let mut current_winning_package_count = u64::MAX;
    let mut current_winning_quantum_entanglement = u64::MAX;

    for first_package_count in 2..cap {
        if first_package_count > current_winning_package_count as usize {
            // it's impossible to find a better solution
            break;
        }

        'start_loop: for first_combination in packages.iter().combinations(first_package_count) {
            let first_size: u64 = first_combination.clone().into_iter().sum();
            if first_size != final_weight {
                continue;
            }

            let remaining_packages_count = packages.len() - first_package_count;
            for second_package_count in (first_package_count + 1)..(remaining_packages_count - 2) {
                let mut remaining_packages = packages.clone();
                remaining_packages.retain(|x| !first_combination.contains(&x));
                for second_combination in remaining_packages.iter().combinations(second_package_count) {
                    let second_size: u64 = second_combination.clone().into_iter().sum();
                    if first_size == second_size {
                        // if not, there's no need to continue
                        let mut third_remaining_packages = packages.clone();
                        third_remaining_packages.retain(|x| !first_combination.contains(&x) && !second_combination.contains(&x));
                        for third_combination in third_remaining_packages.iter().combinations(second_package_count) {
                            let third_size: u64 = third_combination.clone().into_iter().sum();
                            if second_size == third_size {
                                let mut final_remaining_packages = packages.clone();
                                final_remaining_packages.retain(|x| !first_combination.contains(&x) && !second_combination.contains(&x) && !third_combination.contains(&x));
                                if final_remaining_packages.len() <= first_package_count {
                                    continue;
                                }

                                let fourth_size: u64 = final_remaining_packages.clone().into_iter().sum();   
                                if third_size == fourth_size {
                                    if first_package_count <= current_winning_package_count as usize {
                                        let current_quantum_entanglement = first_combination.clone().into_iter().product();
                                        if current_quantum_entanglement < current_winning_quantum_entanglement {
                                            current_winning_package_count = first_package_count as u64;
                                            current_winning_quantum_entanglement = current_quantum_entanglement;
                                            println!("Found winner. Size: {}, QE: {}", current_winning_package_count, current_winning_quantum_entanglement);
                                            
                                            // all other combinations with the first three sets will match, so we break early
                                            break 'start_loop;
                                        }
                                    }
                                }
                            }
                        }
                    }   
                }
            }
        }        
    }

    println!("Lowest balanced quantum entanglement: {}", current_winning_quantum_entanglement);
}