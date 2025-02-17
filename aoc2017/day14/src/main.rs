fn main() {
    let input = "ugkiagan-".to_string();
    let mut hashes = vec![];
    let additional_bytes = vec![17usize, 31usize, 73usize, 47usize, 23usize];    
    for mut row_index in 0..=127 {
        let mut index: usize = 0;
        let mut skip_size = 0;
        let mut data: Vec<_> = (0..=255).collect();
        let mut lengths = vec![];
        
        let appended_input = input.clone().to_owned() + &row_index.to_string();
        for ch in appended_input.chars() {
            lengths.push(ch as usize);
        }
        for byte in &additional_bytes {
            lengths.push(*byte);
        }

        for _ in 1..=64 {
            for len in &lengths {
                let len = *len as usize;
                let start = index;
                let end = (index + len - 1) % data.len();  // -1 since we're 0 based
                reverse(&mut data, start, end, len / 2);
        
                index += len;
                index += skip_size;
                if index > data.len() - 1 {
                    index = index % data.len();
                }
        
                skip_size += 1;
            }
        }

        let mut hash = "".to_string();
        let mut hashindex = 0;
        let mut hashsubindex = 0;
        for hashindex in 0..16 {
            let mut accumulator = 0;
            for hashsubindex in 0..16 {
                if hashsubindex == 0 {
                    accumulator = data[(hashindex * 16) + hashsubindex];
                } else {
                    accumulator ^= data[(hashindex * 16) + hashsubindex];
                }
            }

            hash.push_str(&format!("{:8b}", accumulator));
        }
        
        hashes.push(hash);
    }
    
    /* Part 1
    let mut total = 0;
    for hash in hashes {
        for ch in hash.chars() {
            if ch == '1' {
                total += 1;
            }
        }
    }

    println!("Total: {}", total);
    */

    let mut grid = vec![];
    for hash in hashes {
        let x = hash.len();
        for ch in hash.chars() {
            if ch == '1' {
                grid.push('#');
            } else {
                grid.push('.');
            }
        }
    }

    let mut visited = vec![];
    let mut groups = 0;
    for index in 0..(128 * 128) {
        if grid[index] == '#' {
            if !visited.contains(&index) {
                find_path(&mut grid, &mut visited, index);
                groups += 1;
                
                /*
                print_grid(&grid);
                println!("");
                
                let mut buffer = "".to_string();
                std::io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read line");
                */
            }
        }
    }

    println!("Groups: {}", groups);
}

fn reverse(data: &mut Vec<u8>, mut start: usize, mut end: usize, length: usize) {
    let mut index = 0;
    
    loop {
        if index == length {
            break;
        }

        let temp = data[start];
        data[start] = data[end];
        data[end] = temp;

        start += 1;
        if start == data.len() {
            start = 0;
        }

        if end == 0 {
            end = data.len() - 1;
        } else {
            end -= 1;
        }

        index += 1;
    }
}

fn find_path(grid: &mut Vec<char>, visited: &mut Vec<usize>, index: usize) {
    let mut queue = vec![];
    queue.push(index);

    while queue.len() > 0 {
        let index = queue.pop().unwrap();
        visited.push(index);
        grid[index] = 'X';
        
        // up
        if index >= 128 {
            let offset = index - 128;
            if grid[offset] == '#' && !visited.contains(&offset) {
                queue.push(offset);
            }
        }

        // down
        if index < (128 * 128) - 128 {
            let offset = index + 128;
            if grid[offset] == '#' && !visited.contains(&offset) {
                queue.push(offset);
            }
        }

        // left
        if index % 128 != 0 {
            let offset = index - 1;
            if grid[offset] == '#' && !visited.contains(&offset) {
                queue.push(offset);
            }
        }

        // right
        if index == 0 || ((index + 1) % 128 != 0) {
            let offset = index + 1;
            if grid[offset] == '#' && !visited.contains(&offset) {
                queue.push(offset);
            }
        }
    }
}

fn print_grid(grid: &Vec<char>) {
    let mut index = 1;
    for ch in grid {
        print!("{}", ch);
        if index % 128 == 0 {
            println!("");
        }

        index += 1;
    }
}