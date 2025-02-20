fn main() {
    let iterations = 12964419;
    let mut current_state = 'A';
    let mut tape = vec![0];
    let mut index = 0;

    for i in 1..=iterations {
        match current_state {
            'A' => {
                if tape[index] == 0 {
                    tape[index] = 1;
                    move_tape(&mut tape, &mut index, 'R');
                    current_state = 'B';
                } else {
                    tape[index] = 0;
                    move_tape(&mut tape, &mut index, 'R');
                    current_state = 'F';
                }
            },
            'B' => {
                if tape[index] == 0 {
                    move_tape(&mut tape, &mut index, 'L');
                } else {
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'C';
                }
            },
            'C' => {
                if tape[index] == 0 {
                    tape[index] = 1;
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'D';
                } else {
                    tape[index] = 0;
                    move_tape(&mut tape, &mut index, 'R');
                }
            },
            'D' => {
                if tape[index] == 0 {
                    tape[index] = 1;
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'E';
                } else {
                    move_tape(&mut tape, &mut index, 'R');
                    current_state = 'A';
                }
            },
            'E' => {
                if tape[index] == 0 {
                    tape[index] = 1;
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'F';
                } else {
                    tape[index] = 0;
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'D';
                }
            },
            'F' => {
                if tape[index] == 0 {
                    tape[index] = 1;
                    move_tape(&mut tape, &mut index, 'R');
                    current_state = 'A';
                } else {
                    tape[index] = 0;
                    move_tape(&mut tape, &mut index, 'L');
                    current_state = 'E';
                }
            },
            _ => panic!("Unknown state"),
        }
    }

    println!("1s: {}", tape.iter().filter(|&n| *n == 1).count());
}

fn move_tape(tape: &mut Vec<u8>, index: &mut usize, direction: char) {
    if direction == 'R' {
        if *index == tape.len() - 1 {
            tape.push(0);
        }

        *index += 1;
    } else {
        if *index == 0 {
            tape.insert(0, 0);
            // we don't change index and leave it at 0
        } else {
            *index -= 1;
        }
    }
}