use std::fs::File;
use std::io::Read;

extern crate serde_json;
use serde_json::Value;

fn main() {
    let filename = "input.txt";
    let mut data = vec![];
    let _ = File::open(&filename).and_then(|mut f| f.read_to_end(&mut data));
    let json: Value = serde_json::from_slice(&data[0..data.len()]).unwrap();
    
    println!("Total: {}", get_sum(json));    
}

/* Day1
fn get_sum(value: Value) -> i64 {
    let mut total = 0;

    if value.is_number() {
        total += value.as_i64().unwrap();
    } else if value.is_object() {
        for object in value.as_object().unwrap() {
            let (subkey, subvalue) = object;
            total += get_sum(subvalue.clone());
        }
    } else if value.is_array() {
        for array in value.as_array() {
            for object in array {
                total += get_sum(object.clone());
            }
        }
    }

    total
}
*/

fn get_sum(value: Value) -> i64 {
    let mut total = 0;

    if value.is_number() {
        total += value.as_i64().unwrap();
    } else if value.is_object() {
        let has_red: Vec<_> = value.as_object().unwrap().iter().filter(|(k,v)| *v == "red").collect();
        if has_red.len() > 0 {
            return 0;
        }

        for object in value.as_object().unwrap() {
            let (subkey, subvalue) = object;
            total += get_sum(subvalue.clone());
        }
    } else if value.is_array() {
        for array in value.as_array() {
            for object in array {
                total += get_sum(object.clone());
            }
        }
    }

    total
}