#[derive(Clone)]
enum Spell {
    None,
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
struct GameState {
    pub player_hp: i16,
    pub player_mana: i16,
    pub player_armor: i16,
    pub is_shielded: bool,
    pub shield_timer: u8,
    pub is_recharging: bool,
    pub recharge_timer: u8,
    pub boss_hp: i16,
    pub is_poisoned: bool,
    pub poison_timer: u8,
    pub total_mana: i16,
    pub spell: Spell,
    pub player_lost: bool,
}

const HARD_MODE: bool = true;

fn main() {
    let game_state = GameState {
        player_hp: 50,
        player_mana: 500,
        player_armor: 0,
        is_shielded: false,
        shield_timer: 0,
        is_recharging: false,
        recharge_timer: 0,
        boss_hp: 51,
        is_poisoned: false,
        poison_timer: 0,
        total_mana: 0,
        spell: Spell::None,
        player_lost: false,
    };

    let mut starting_magic_missile = game_state.clone();
    starting_magic_missile.spell = Spell::MagicMissile;
    let mut starting_drain = game_state.clone();
    starting_drain.spell = Spell::Drain;
    let mut starting_shield = game_state.clone();
    starting_shield.spell = Spell::Shield;
    let mut starting_poison = game_state.clone();
    starting_poison.spell = Spell::Poison;
    let mut starting_recharge = game_state.clone();
    starting_recharge.spell = Spell::Recharge;
    
    let lowest_wins = vec![
        get_lowest_mana_win(&mut starting_magic_missile),
        get_lowest_mana_win(&mut starting_drain),
        get_lowest_mana_win(&mut starting_shield),
        get_lowest_mana_win(&mut starting_poison),
        get_lowest_mana_win(&mut starting_recharge),
    ];
    
    let lowest_win = lowest_wins.into_iter().min().unwrap();

    println!("Lowest win: {}", lowest_win);
}

fn get_lowest_mana_win(game_state: &mut GameState) -> i16 {
    // first we tick the game
    tick_game(game_state);

    if game_state.player_lost || game_state.player_hp <= 0 || game_state.player_mana <= 0 {
        return i16::MAX;
    } else if game_state.boss_hp <= 0 {
        return game_state.total_mana;
    }

    let mut next_moves = vec![];

    // if we're not done, we recursively run the game for each valid command
    if game_state.player_mana >= 53 || (game_state.is_recharging){
        let mut magic_missile_next = game_state.clone();
        magic_missile_next.spell = Spell::MagicMissile;
        next_moves.push(magic_missile_next);    
    }

    if game_state.player_mana >= 73 || (game_state.is_recharging) {
        let mut drain_next = game_state.clone();
        drain_next.spell = Spell::Drain;
        next_moves.push(drain_next);    
    }

    if !game_state.is_shielded && (game_state.player_mana >= 113 || (game_state.is_recharging && game_state.player_mana + 101 >= 113)) {
        let mut shield_next = game_state.clone();
        shield_next.spell = Spell::Shield;
        next_moves.push(shield_next);    
    }

    if !game_state.is_poisoned && game_state.player_mana >= 173 || ((game_state.is_recharging && game_state.player_mana + 101 >= 173)) {
        let mut poison_next = game_state.clone();
        poison_next.spell = Spell::Poison;
        next_moves.push(poison_next);    
    }

    if !game_state.is_recharging && game_state.player_mana >= 229 {
        let mut recharge_next = game_state.clone();
        recharge_next.spell = Spell::Recharge;
        next_moves.push(recharge_next);    
    }

    let mut results = vec![];
    for mut next_move in next_moves {
        results.push(get_lowest_mana_win(&mut next_move));
    }

    // if there are no moves, we ran out of mana and return the MAX
    match results.into_iter().min() {
        Some(min) => min,
        None => i16::MAX,
    }
}

fn tick_game(game_state: &mut GameState) {
    // Part 2: -1 player hp on each turn
    if HARD_MODE {
        game_state.player_hp -= 1;
        if game_state.player_hp <= 0 {
            game_state.player_lost = true;
        }
    }

    // we handle any effects
    handle_effects(game_state);

    // handle the player's turn
    match game_state.spell {
        Spell::None => panic!("None should never be executed!"),
        Spell::MagicMissile => {
            game_state.player_mana -= 53;
            game_state.total_mana += 53;
            game_state.boss_hp -= 4;
        },
        Spell::Drain => {
            game_state.player_mana -= 73;
            game_state.total_mana += 73;
            game_state.boss_hp -= 2;
            game_state.player_hp += 2;
        },
        Spell::Shield => {
            game_state.player_mana -= 113;
            game_state.total_mana += 113;
            shield_player(game_state);
        },
        Spell::Poison => {
            game_state.player_mana -= 173;
            game_state.total_mana += 173;
            poison_boss(game_state);
        },
        Spell::Recharge => {
            game_state.player_mana -= 229;
            game_state.total_mana += 229;
            recharge_player(game_state);
        },
    };

    // we handle any effects on the boss' turn
    handle_effects(game_state);
    if game_is_over(game_state) {
        // the boss died from poison
        return;
    }

    // handle the boss' turn
    game_state.player_hp -= 9 - game_state.player_armor;
}

fn handle_effects(game_state: &mut GameState) {
    if game_state.is_poisoned {
        game_state.boss_hp -= 3;
        game_state.poison_timer -= 1;
        if game_state.poison_timer == 0 {
            game_state.is_poisoned = false;
        }
    }

    if game_state.is_shielded {
        game_state.shield_timer -= 1;
        if game_state.shield_timer == 0 {
            game_state.is_shielded = false;
            game_state.player_armor -= 7;
        } 
    }

    if game_state.is_recharging {
        game_state.player_mana += 101;
        game_state.recharge_timer -= 1;
        if game_state.recharge_timer == 0 {
            game_state.is_recharging = false;
        } 
    }
}

fn poison_boss(game_state: &mut GameState) {
    game_state.is_poisoned = true;
    game_state.poison_timer = 6;
}

fn shield_player(game_state: &mut GameState) {
    game_state.is_shielded = true;
    game_state.player_armor += 7;
    game_state.shield_timer = 6;
}

fn recharge_player(game_state: &mut GameState) {
    game_state.is_recharging = true;
    game_state.recharge_timer = 5;
}

fn game_is_over(game_state: &GameState) -> bool {
    game_state.boss_hp <= 0 || game_state.player_hp <= 0 || game_state.player_mana <= 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_poison_set_and_effects() {
        let mut game_state = GameState {
            player_hp: 50,
            player_mana: 500,
            player_armor: 0,
            is_shielded: false,
            shield_timer: 0,
            is_recharging: false,
            recharge_timer: 0,
            boss_hp: 51,
            is_poisoned: false,
            poison_timer: 0,
            total_mana: 0,
            spell: Spell::None,
        };

        poison_boss(&mut game_state);
        assert_eq!(game_state.boss_hp, 51);
        assert_eq!(game_state.is_poisoned, true);
        assert_eq!(game_state.poison_timer, 6);

        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);

        assert_eq!(game_state.boss_hp, 51 - 6 * 3);
        assert_eq!(game_state.is_poisoned, false);
        assert_eq!(game_state.poison_timer, 0);
    }

    #[test]
    fn test_shield_set_and_effects() {
        let mut game_state = GameState {
            player_hp: 50,
            player_mana: 500,
            player_armor: 0,
            is_shielded: false,
            shield_timer: 0,
            is_recharging: false,
            recharge_timer: 0,
            boss_hp: 51,
            is_poisoned: false,
            poison_timer: 0,
            total_mana: 0,
            spell: Spell::None,
        };

        shield_player(&mut game_state);
        assert_eq!(game_state.player_armor, 7);
        assert_eq!(game_state.is_shielded, true);
        assert_eq!(game_state.shield_timer, 6);

        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);

        assert_eq!(game_state.player_armor, 0);
        assert_eq!(game_state.is_shielded, false);
        assert_eq!(game_state.shield_timer, 0);
    }

    #[test]
    fn test_recharge_set_and_effects() {
        let mut game_state = GameState {
            player_hp: 50,
            player_mana: 500,
            player_armor: 0,
            is_shielded: false,
            shield_timer: 0,
            is_recharging: false,
            recharge_timer: 0,
            boss_hp: 51,
            is_poisoned: false,
            poison_timer: 0,
            total_mana: 0,
            spell: Spell::None,
        };

        recharge_player(&mut game_state);
        assert_eq!(game_state.is_recharging, true);
        assert_eq!(game_state.recharge_timer, 5);
        assert_eq!(game_state.player_mana, 500); // it shouldn't change at this point

        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);
        handle_effects(&mut game_state);

        assert_eq!(game_state.player_mana, 1005);
        assert_eq!(game_state.is_recharging, false);
        assert_eq!(game_state.recharge_timer, 0);
    }


    #[test]
    fn test_tick_game() {
        let mut game_state = GameState {
            player_hp: 10,
            player_mana: 250,
            player_armor: 0,
            is_shielded: false,
            shield_timer: 0,
            is_recharging: false,
            recharge_timer: 0,
            boss_hp: 13,
            is_poisoned: false,
            poison_timer: 0,
            total_mana: 0,
            spell: Spell::Poison,
        };

        tick_game(&mut game_state);

        assert_eq!(game_state.player_hp, 2);
        assert_eq!(game_state.player_mana, 77);
        assert_eq!(game_state.boss_hp, 10);
        assert_eq!(game_state.is_poisoned, true);
        assert_eq!(game_state.poison_timer, 5);
    }
}