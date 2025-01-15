fn main() {
    let row = 2981;
    let column = 3075;
    let mut cell: u128 = 0;
    for x in 1..=(row - 1) {
        cell += x;
    }

    cell += 1;
    let mut offset: u128 = row + 1;
    
    for y in 1..column {
        cell += offset;
        offset += 1;
    }

    // iteration cell is our code
    let mut code: u128 = 20151125;
    for _ in 1..cell {
        code = (code * 252533) % 33554393;
    }

    println!("Final code: {}", code);
}