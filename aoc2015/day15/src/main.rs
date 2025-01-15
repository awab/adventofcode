use std::collections::HashMap;
use std::fs::read_to_string;

use regex::Regex;

const MAX_TEASPOONS: i32 = 100;
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let filename = "input.txt";
    let lines: Vec<String> = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let pattern = Regex::new(r"([^:]+):\scapacity\s(-?\d+),\sdurability\s(-?\d+),\sflavor\s(-?\d+),\stexture\s(-?\d+),\scalories\s(-?\d+)").unwrap();

    let mut all_ingredients = HashMap::new();

    for line in lines {
        for (_, [name, capacity, durability, flavor, texture, calories]) in pattern.captures_iter(&line).map(|c| c.extract()) {
            let ingredient = Ingredient {
                name: name.clone().to_string(),
                capacity: capacity.parse::<i32>().unwrap(),
                durability: durability.parse::<i32>().unwrap(),
                flavor: flavor.parse::<i32>().unwrap(),
                texture: texture.parse::<i32>().unwrap(),
                calories: calories.parse::<i32>().unwrap(),
            };
            
            all_ingredients.insert(name.clone().to_string(), ingredient);
        }
    }

    let mut all_combinations = vec![];
    for sprinkles in 0..=MAX_TEASPOONS {
        let remaining1 = MAX_TEASPOONS - sprinkles;
        for butterscotch in 0..=remaining1 {
            let remaining2 = MAX_TEASPOONS - sprinkles - butterscotch;
            for chocolate in 0..=remaining2 {
                let candy = MAX_TEASPOONS - sprinkles - butterscotch - chocolate;
                all_combinations.push((sprinkles, butterscotch, chocolate, candy));
            }
        }
    }
    let sprinkles_ingredient = all_ingredients.get(&String::from("Sprinkles")).unwrap();
    let butterscotch_ingredient = all_ingredients.get(&String::from("Butterscotch")).unwrap();
    let chocolate_ingredient = all_ingredients.get(&String::from("Chocolate")).unwrap();
    let candy_ingredient = all_ingredients.get(&String::from("Candy")).unwrap();
    
    let mut max_score: i64 = 0;

    for combination in all_combinations {
        let (sprinkles, butterscotch, chocolate, candy) = combination;
        let mut capacity = (sprinkles * sprinkles_ingredient.capacity) + (butterscotch * butterscotch_ingredient.capacity) + (chocolate * chocolate_ingredient.capacity) + (candy * candy_ingredient.capacity);
        if capacity < 0 {
            capacity = 0;
        }

        let mut durability = (sprinkles * sprinkles_ingredient.durability) + (butterscotch * butterscotch_ingredient.durability) + (chocolate * chocolate_ingredient.durability) + (candy * candy_ingredient.durability);
        if durability < 0 {
            durability = 0;
        }

        let mut flavor = (sprinkles * sprinkles_ingredient.flavor) + (butterscotch * butterscotch_ingredient.flavor) + (chocolate * chocolate_ingredient.flavor) + (candy * candy_ingredient.flavor);
        if flavor < 0 {
            flavor = 0;
        }

        let mut texture = (sprinkles * sprinkles_ingredient.texture) + (butterscotch * butterscotch_ingredient.texture) + (chocolate * chocolate_ingredient.texture) + (candy * candy_ingredient.texture);
        if texture < 0 {
            texture = 0;
        }

        let mut calories = (sprinkles * sprinkles_ingredient.calories) + (butterscotch * butterscotch_ingredient.calories) + (chocolate * chocolate_ingredient.calories) + (candy * candy_ingredient.calories);
        if calories < 0 {
            calories = 0;
        }
        
        if calories == 500 {
            let score: i64 = (capacity as i64) * (durability as i64) * (flavor as i64) * (texture as i64);
            if score > max_score {
                max_score = score;
            }    
        }
    }

    println!("Max cookie score: {}", max_score);
}