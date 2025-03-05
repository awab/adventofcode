use std::fs::read_to_string;
use std::io::{stdin, Read};

use find_all::FindAll;

const GRID_WIDTH: usize = 150;
const GRID_HEIGHT: usize = 150;

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum Turn {
    Left,
    Right,
    Straight,
}

#[derive(PartialEq)]
pub struct Cart {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub turn: Turn,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let mut track = vec![];
    let mut carts = vec![];
    let mut index = 0;
    
    for line in lines {
        for ch in line.chars() {
            match ch {
                '^' | 'v' | '>' | '<' => {
                    // we found a cart
                    let x = index % GRID_WIDTH;
                    let y = index / GRID_WIDTH;
                    let direction = 
                        if ch == '^' {
                            Direction::Up
                        } else if ch == 'v' {
                            Direction::Down
                        } else if ch == '>' {
                            Direction::Right
                        } else {
                            Direction::Left
                        };

                    let cart = Cart {
                        x: x,
                        y: y,
                        direction: direction,
                        turn: Turn::Left,
                    };

                    carts.push(cart);
                    if ch == '^' || ch == 'v' {
                        track.push('|');
                    } else {
                        track.push('-');
                    }
                },
                _ => track.push(ch),
            }

            index += 1;
        }
    }

    let mut step = 0;

    loop {
        // carts are handled from the top row down per the instructions
        carts.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        let mut index = 0;
        let mut max_index = carts.len();
        while index < max_index {
            // check for any collisions after any cart has moved
            let collision = carts
            .iter()
            .enumerate()
            .find_map(|(i, v1)| carts[i + 1..]
                    .iter()
                    .find(|v2| v1.x == v2.x && v1.y == v2.y))
                    .and_then(|v| Some(v));

            if collision != None {
                let collision_details = collision.unwrap();
                /* Part 1 is the first collision */
                println!("Collision at {}: {},{}", step, collision_details.x, collision_details.y);
                let carts_to_remove = carts.iter().find_all(|x| x.x == collision_details.x && x.y == collision_details.y).unwrap();
                for cart_index in carts_to_remove.iter().rev() {
                    carts.remove(*cart_index);
                    if *cart_index < index {
                        index -= 1;
                    }
                }

                max_index = carts.len();
            }

            let mut cart = &mut carts[index];
            if cart.direction == Direction::Up {
                cart.y -= 1;
            } else if cart.direction == Direction::Down {
                cart.y += 1;
            } else if cart.direction == Direction::Left {
                cart.x -= 1;
            } else if cart.direction == Direction::Right {
                cart.x += 1;
            }

            let track_index = (cart.y * GRID_WIDTH) + cart.x;
            let current_track = track[track_index];
            if current_track != '|' && current_track != '-' {
                // if we're going straight, we don't do anything else
                if current_track == '/' {
                    if cart.direction == Direction::Up {
                        cart.direction = Direction::Right;
                    } else if cart.direction == Direction::Down {
                        cart.direction = Direction::Left;
                    } else if cart.direction == Direction::Left {
                        cart.direction = Direction::Down;
                    } else if cart.direction == Direction::Right {
                        cart.direction = Direction::Up;
                    }
                } else if current_track == '\\' {
                    if cart.direction == Direction::Up {
                        cart.direction = Direction::Left;
                    } else if cart.direction == Direction::Down {
                        cart.direction = Direction::Right;
                    } else if cart.direction == Direction::Right {
                        cart.direction = Direction::Down;
                    } else if cart.direction == Direction::Left {
                        cart.direction = Direction::Up;
                    }
                } else if current_track == '+' {
                    // do the turn
                    cart.direction = do_turn(&cart.turn, &cart.direction);

                    // let's rotate the turn
                    if cart.turn == Turn::Left {
                        cart.turn = Turn::Straight;
                    } else if cart.turn == Turn::Straight {
                        cart.turn = Turn::Right;
                    } else if cart.turn == Turn::Right {
                        cart.turn = Turn::Left;
                    }
                }
            }

            index += 1;
        }

        if carts.len() == 1 {
            println!("1 cart remains at: {},{}", carts[0].x, carts[0].y);
            return;
        }

        step += 1;

        /*
        draw_track(&track, &carts);
        let _ = stdin().read(&mut [0u8]).unwrap();
        */
    }
}

fn do_turn(turn: &Turn, direction: &Direction) -> Direction {
    let mut result = direction.clone();

    if *turn == Turn::Left {
        if *direction == Direction::Up {
            result = Direction::Left;
        } else if *direction == Direction::Down {
            result = Direction::Right;
        } else if *direction == Direction::Left {
            result = Direction::Down;
        } else if *direction == Direction::Right {
            result = Direction::Up;
        }
    } else if *turn == Turn::Right {
        if *direction == Direction::Up {
            result = Direction::Right;
        } else if *direction == Direction::Down {
            result = Direction::Left;
        } else if *direction == Direction::Left {
            result = Direction::Up;
        } else if *direction == Direction::Right {
            result = Direction::Down;
        }
    }
    // if it's straight, we do nothing

    result.clone()
}

fn draw_track(track: &Vec<char>, carts: &Vec<Cart>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let cart = carts.iter().find(|c| c.x == x && c.y == y);
            if cart == None {
                let index = (y * GRID_WIDTH) + x;
                print!("{}", track[index])    
            } else {
                let cart = cart.unwrap();
                let direction = 
                    if cart.direction == Direction::Up {
                        '^'
                    } else if cart.direction == Direction::Down {
                        'v'
                    } else if cart.direction == Direction::Left {
                        '<'
                    } else {
                        '>'
                    };
                print!("{}", direction);
            }
        }

        println!("");
    }
}