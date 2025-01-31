/* Part 1
fn main() {    
    let elf_count = 3017957;
    let mut elves: Vec<u32> = vec![];
    for _ in 0..elf_count {
        elves.push(1);
    }

    'white_elephant: loop {
        for index in 0..elf_count {
            // only elves with gifts can play
            if elves[index] > 0 {
                let mut next_with_gifts = None;
                for next_with_gifts_index in (index + 1)..elf_count {
                    if elves[next_with_gifts_index] > 0 {
                        next_with_gifts = Some(next_with_gifts_index);
                        break;
                    }
                }

                if next_with_gifts == None {
                    // we have to check again from the start
                    for next_with_gifts_index in 0..index {
                        if elves[next_with_gifts_index] > 0 {
                            next_with_gifts = Some(next_with_gifts_index);
                            break;
                        }    
                    }
                }
    
                if next_with_gifts == None {
                    println!("Winner: {}", index + 1);
                    return;
                } else {
                    let next_with_gifts = next_with_gifts.unwrap();
                    elves[index] += elves[next_with_gifts];
                    elves[next_with_gifts] = 0;        
                }
            }
        }
    }
}
*/


fn main() {    
    let elf_count = 3017957;
    //let elf_count = 3014387; //1420064
    //let elf_count = 5; // 2
    let mut elves_position: Vec<u32> = vec![];
    for index in 1..=elf_count {
        elves_position.push(index);
    }

    let mut index: i32 = -1;
    while elves_position.len() > 1 {
        index += 1;
        if index == elves_position.len() as i32 {
            index = 0;
        }

        let mut offset = elves_position.len() / 2;
        offset += index as usize;
        offset %= elves_position.len();
        //println!("{} takes from {}", elves_position[index as usize], elves_position[offset]);
        elves_position.remove(offset);

        if index as usize > offset {
            index -= 1;
        }
    }

    println!("Winner: {}", elves_position[0]);
}