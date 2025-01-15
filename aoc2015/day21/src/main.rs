use itertools::Itertools;

struct Weapon {
    pub name: String,
    pub cost: u16,
    pub damage: u8,
}

struct Armor {
    pub name: String,
    pub cost: u16,
    pub armor: u8,
}

#[derive(Clone)]
struct Ring {
    pub name: String,
    pub cost: u16,
    pub damage: u8,
    pub armor: u8,
}

fn main() {
    // setup the shop
    let mut weapons = vec![];
    weapons.push(Weapon {
        name: String::from("Dagger"),
        cost: 8,
        damage: 4,
    });
    weapons.push(Weapon {
        name: String::from("Shortsword"),
        cost: 10,
        damage: 5,
    });
    weapons.push(Weapon {
        name: String::from("Warhammer"),
        cost: 25,
        damage: 6,
    });
    weapons.push(Weapon {
        name: String::from("Longsword"),
        cost: 40,
        damage: 7,
    });
    weapons.push(Weapon {
        name: String::from("Greataxe"),
        cost: 74,
        damage: 8,
    });

    let mut armors = vec![];
    armors.push(Armor {
        name: String::from("None"),
        cost: 0,
        armor: 0,
    });
    armors.push(Armor {
        name: String::from("Leather"),
        cost: 13,
        armor: 1,
    });
    armors.push(Armor {
        name: String::from("Chainmail"),
        cost: 31,
        armor: 2,
    });
    armors.push(Armor {
        name: String::from("Splintmail"),
        cost: 53,
        armor: 3,
    });
    armors.push(Armor {
        name: String::from("Bandedmail"),
        cost: 75,
        armor: 4,
    });
    armors.push(Armor {
        name: String::from("Platemail"),
        cost: 102,
        armor: 5,
    });

    let mut rings = vec![];
    rings.push(Ring {
        name: String::from("None"),
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Ring {
        name: String::from("Damage +1"),
        cost: 25,
        damage: 1,
        armor: 0,
    });
    rings.push(Ring {
        name: String::from("Damage +2"),
        cost: 50,
        damage: 2,
        armor: 0,
    });
    rings.push(Ring {
        name: String::from("Damage +3"),
        cost: 100,
        damage: 3,
        armor: 0,
    });
    rings.push(Ring {
        name: String::from("Defense +1"),
        cost: 20,
        damage: 0,
        armor: 1,
    });
    rings.push(Ring {
        name: String::from("Defense +2"),
        cost: 40,
        damage: 0,
        armor: 2,
    });
    rings.push(Ring {
        name: String::from("Defense +3"),
        cost: 80,
        damage: 0,
        armor: 3,
    });

    let mut winner_costs = vec![];
    let ring_combinations: Vec<_> = rings.clone().into_iter().combinations(2).collect();
    // iterate through every combination and track winners
    for weapon in &weapons {
        let mut total_cost = 0;
        total_cost += weapon.cost;
        
        for armor in &armors {
            let mut total_cost = total_cost;
            total_cost += armor.cost;

            for ring in &rings {
                // we try each ring on its own, including the None option
                let mut total_cost = total_cost;
                total_cost += ring.cost;
                let winner = play_game(weapon, armor, &vec![ring.clone()]);
                if !winner {
                    winner_costs.push(total_cost);
                }
            }

            for ring_combination in &ring_combinations {
                // we try each ring combination of 1 (since there is a None) or 2
                let mut total_cost = total_cost;
                total_cost += ring_combination[0].cost + ring_combination[1].cost;
                let winner = play_game(weapon, armor, ring_combination);
                if !winner {
                    winner_costs.push(total_cost);
                }
            }
        }
    }

    println!("Highest cost win: {}", winner_costs.iter().max().unwrap());
}

fn play_game(weapon: &Weapon, armor: &Armor, rings: &Vec<Ring>) -> bool {
    let mut game_won = false;
    let mut player_hp: i8 = 100;
    let mut player_damage = weapon.damage;
    let mut player_armor = armor.armor;
    let mut boss_hp: i8 = 103;
    let boss_damage = 9;
    let boss_armor = 2;

    // add the modifiers from the rings
    for ring in rings {
        player_damage += ring.damage;
        player_armor += ring.armor;
    }

    while player_hp > 0 && boss_hp > 0 {
        // player's turn
        let mut damage = (player_damage as i8) - (boss_armor as i8);
        if damage <= 0 {
            damage = 1;
        }
        boss_hp -= damage;
        if boss_hp <= 0 {
            game_won = true;
            break;
        }

        damage = (boss_damage as i8) - (player_armor as i8);
        if damage <= 0 {
            damage = 1;
        }
        player_hp -= damage;
        if player_hp <= 0 {
            // game_won is already false
            break;
        }
    }
    
    game_won
}