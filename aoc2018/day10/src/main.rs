use std::fs::read_to_string;

use kdtree::KdTree;
use num_traits::Float;
use regex::Regex;

struct Point {
    pub position_x: f64,
    pub position_y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let re = Regex::new(r"position=<(.*),(.*)>\svelocity=<(.*),(.*)>").unwrap();
    let mut points = vec![];
    for line in lines {
        for (_, [position_x, position_y, velocity_x, velocity_y]) in re.captures_iter(&line).map(|c| c.extract()) {
            let point = Point {
                position_x: position_x.trim().parse::<f64>().unwrap(),
                position_y: position_y.trim().parse::<f64>().unwrap(),
                velocity_x: velocity_x.trim().parse::<f64>().unwrap(),
                velocity_y: velocity_y.trim().parse::<f64>().unwrap(),
            };

            points.push(point);
        }
    }

    let mut steps = 0;
    let dimensions = 2;
    loop {
        let mut tree_list = vec![];
        for index in 0..points.len() {
            tree_list.push([points[index].position_x, points[index].position_y]);
        }
        let mut kdtree = KdTree::new(dimensions);
        for index in 0..tree_list.len() {
            kdtree.add(&tree_list[index], index);
        }

        let mut close_points = 0;
        for point in &points {
            let tree_point = [point.position_x, point.position_y];
            let nearest = kdtree.nearest(&tree_point, 2, &manhattan).unwrap();
            if nearest[0].0 <= 2.0 && nearest[1].0 <= 2.0 {
                // 2 because we're using manhattan and diagonals are 2 spaces away
                close_points += 1;
            }
        }

        // if 90% of the characters are within 2 spaces (1 diagonal), we consider it done
        if close_points > ((tree_list.len() as f64) * 0.9) as usize {
            break;
        }

        for point in &mut points {
            point.position_x += point.velocity_x;
            point.position_y += point.velocity_y;
        }

        steps += 1;
    }

    let mut min_x = f64::MAX;
    let mut max_x = 0f64;
    let mut min_y = f64::MAX;
    let mut max_y = 0f64;

    for point in &points {
        if point.position_x < min_x {
            min_x = point.position_x;
        }

        if point.position_x > max_x {
            max_x = point.position_x;
        }

        if point.position_y < min_y {
            min_y = point.position_y;
        }

        if point.position_y > max_y {
            max_y = point.position_y;
        }
    }

    for y in (min_y as usize)..=(max_y as usize) {
        for x in (min_x as usize)..=(max_x as usize) {
            let x_val = x as f64;
            let y_val = y as f64;
            if points.iter().any(|x| x.position_x == x_val && x.position_y == y_val) {
                print!("#");
            } else {
                print!(".")
            }
        }

        println!("");
    }

    println!("Steps: {}", steps);
}

fn manhattan<T: Float>(a: &[T], b: &[T]) -> T {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| ((*x) - (*y)).abs() + ((*x) - (*y)).abs())
        .fold(T::zero(), ::std::ops::Add::add)
}