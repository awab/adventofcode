use std::fs::read_to_string;

const DISPLAY_WIDTH: usize = 50;
const DISPLAY_HEIGHT: usize = 6;

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut lights: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT] = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];

    for line in lines {
        println!("{}", line);

        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "rect" {
            let rectangle_dimensions: Vec<&str> = parts[1].split('x').collect();
            let width = rectangle_dimensions[0].parse::<u16>().unwrap();
            let height = rectangle_dimensions[1].parse::<u16>().unwrap();
            draw_rectangle(&mut lights, width, height);
        } else if parts[0] == "rotate" {
            let index_parts: Vec<&str> = parts[2].split('=').collect();
            let index = index_parts[1].parse::<u16>().unwrap();
            // parts[3] == "by"
            let amount = parts[4].parse::<u16>().unwrap();
            if parts[1] == "column" {
                rotate_column(&mut lights, index, amount);
            } else {
                rotate_row(&mut lights, index, amount);
            }
        }

        display_lights(&lights);
    }

    let lit_count = get_lit_count(&lights);
    println!("Total lit: {}", lit_count);
}

fn get_lit_count(lights: &[bool]) -> u16 {
    let mut lit_count = 0;

    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let index = (y as usize * DISPLAY_WIDTH) + x as usize;
            if lights[index as usize] {
                lit_count += 1;
            }  
        }
    }

    lit_count
}

fn display_lights(lights: &[bool]) {
    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let index = (y as usize * DISPLAY_WIDTH) + x as usize;
            if index > 0 && (index % DISPLAY_WIDTH) == 0 {
                println!("");
            }

            let ch = match lights[index as usize] {
                true => '#',
                false => '.',
            };

            print!("{}", ch);
        }
    }

    println!("");
    println!("");
}

fn draw_rectangle(lights: &mut [bool], width: u16, height: u16) {
    for y in 0..height {
        for x in 0..width {
            let index: u16 = (y * DISPLAY_WIDTH as u16) + x;
            lights[index as usize] = true;
        }
    }
}

fn rotate_column(lights: &mut [bool], index: u16, amount: u16) {
    let column_start = index as usize;
    let column_end = column_start as usize + ((DISPLAY_HEIGHT - 1) * DISPLAY_WIDTH);
    let mut lights_temp = vec![];
    // we know how many characters will be pushed off the end, let's do those
    for i in 0..(amount as usize) {
        let offset = (column_end - (i * DISPLAY_WIDTH)) as usize;
        lights_temp.push(lights[offset]);
    }

    // now we can simply overwrite the rest
    for i in ((amount as usize)..(DISPLAY_HEIGHT as usize)).rev() {
        let current_index = (i * DISPLAY_WIDTH) + index as usize;
        let replacement_index = ((i - amount as usize) * DISPLAY_WIDTH) + index as usize;
        lights[current_index] = lights[replacement_index];
    }

    // and pop the ones we're shifting
    for i in 0..amount {
        let current_index: usize = (i as usize * DISPLAY_WIDTH) + index as usize;
        lights[current_index] = lights_temp.pop().unwrap();
    }
}

fn rotate_row(lights: &mut [bool], index: u16, amount: u16) {
    let row_start = index as usize * DISPLAY_WIDTH;
    let row_end = row_start as usize + DISPLAY_WIDTH - 1;
    let mut lights_temp = vec![];
    // we know how many characters will be pushed off the end, let's do those
    for i in 0..(amount as usize) {
        let offset = (row_end - i) as usize;
        lights_temp.push(lights[offset]);
    }

    // now we can simply overwrite the rest
    for i in (amount as usize..DISPLAY_WIDTH).rev() {
        let current_index = row_start + i as usize;
        lights[current_index] = lights[current_index - amount as usize];
    }

    // and pop the ones we're shifting
    for i in 0..amount {
        let current_index: usize = row_start + i as usize;
        lights[current_index] = lights_temp.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_and_display() {
        let mut lights: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT] = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        draw_rectangle(&mut lights, 10, 5);
        display_lights(&lights);
    }
}