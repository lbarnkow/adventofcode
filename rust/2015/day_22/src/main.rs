#![allow(dead_code)]

use std::collections::VecDeque;
fn main() {
    println!("Advent of Code 2015 - day 22");
}

#[derive(Debug, Clone)]
struct Spell {
    name: &'static str,
    cost: i64,
    immediate: bool,
    duration: usize,
    damage: i64,
    heal: i64,
    armor: i64,
    mana: i64,
}

#[derive(Debug, Clone)]
struct Char {
    hit_points: i64,
    mana: i64,
    damage: i64,
}

#[derive(Debug, Clone)]
struct Battle {
    player: Char,
    boss: Char,
    turn: usize,
    mana_spent: i64,
    damange_dealt: i64,
    active_spells: VecDeque<Spell>,
}

fn initialize_chars(file: &str, p_hp: i64, p_mana: i64) -> (Char, Char) {
    let mut boss = Char {
        hit_points: 0,
        mana: 0,
        damage: 0,
    };

    let file = std::fs::read_to_string(file).unwrap();
    for line in file.lines() {
        let mut split = line.split(':');
        match split.next().unwrap().trim() {
            "Hit Points" => boss.hit_points = split.next().unwrap().trim().parse().unwrap(),
            "Damage" => boss.damage = split.next().unwrap().trim().parse().unwrap(),
            _ => panic!("illegal prop"),
        }
    }

    let player = Char {
        hit_points: p_hp,
        mana: p_mana,
        damage: 0,
    };

    (player, boss)
}

fn initialize_battle(player: Char, boss: Char) -> Battle {
    Battle {
        player,
        boss,
        turn: 0,
        mana_spent: 0,
        damange_dealt: 0,
        active_spells: VecDeque::new(),
    }
}

fn initialize_spells() -> Vec<Spell> {
    vec![
        Spell {
            name: "Magic Missile",
            cost: 53,
            immediate: true,
            duration: 0,
            damage: 4,
            heal: 0,
            armor: 0,
            mana: 0,
        },
        Spell {
            name: "Drain",
            cost: 73,
            immediate: true,
            duration: 0,
            damage: 2,
            heal: 2,
            armor: 0,
            mana: 0,
        },
        Spell {
            name: "Shield",
            cost: 113,
            immediate: false,
            duration: 6,
            damage: 0,
            heal: 0,
            armor: 7,
            mana: 0,
        },
        Spell {
            name: "Poison",
            cost: 173,
            immediate: false,
            duration: 6,
            damage: 3,
            heal: 0,
            armor: 0,
            mana: 0,
        },
        Spell {
            name: "Recharge",
            cost: 229,
            immediate: false,
            duration: 5,
            damage: 0,
            heal: 0,
            armor: 0,
            mana: 101,
        },
    ]
}

#[derive(Debug)]
struct BattleResult {
    player_won: bool,
    turns: usize,
    mana_spent: i64,
}

fn win_with_least_amount_of_mana_spent(
    state: Battle,
    spells: &Vec<Spell>,
    hard: bool,
) -> BattleResult {
    let mut q = Vec::new();

    let mut best = BattleResult {
        player_won: false,
        turns: usize::MAX,
        mana_spent: i64::MAX,
    };

    q.push(state);

    while let Some(mut state) = q.pop() {
        if state.mana_spent >= best.mana_spent {
            continue;
        }
        state.turn += 1;

        let mut player_armor = 0;

        for active_spell in state.active_spells.iter_mut() {
            active_spell.duration -= 1;
            player_armor += active_spell.armor;
            state.boss.hit_points -= active_spell.damage;
            state.damange_dealt += active_spell.damage;
            state.player.mana += active_spell.mana;
        }
        while !state.active_spells.is_empty() {
            if state.active_spells.get(0).unwrap().duration > 0 {
                break;
            }
            state.active_spells.pop_front();
        }

        if hard && state.turn % 2 == 1 {
            state.player.hit_points -= 1
        }
        if state.player.hit_points <= 0 {
            continue;
        }
        if state.boss.hit_points <= 0 {
            if state.mana_spent < best.mana_spent {
                best = BattleResult {
                    player_won: true,
                    turns: state.turn,
                    mana_spent: state.mana_spent,
                };
            }
            continue;
        }

        if state.turn % 2 == 1 {
            for spell in spells.iter() {
                if spell.cost > state.player.mana {
                    continue;
                }
                if state
                    .active_spells
                    .iter()
                    .filter(|x| x.name == spell.name)
                    .count()
                    != 0
                {
                    continue;
                }
                let mut state = state.clone();
                state.player.mana -= spell.cost;
                state.mana_spent += spell.cost;
                if spell.immediate {
                    state.boss.hit_points -= spell.damage;
                    state.damange_dealt += spell.damage;
                    state.player.hit_points += spell.heal;
                } else {
                    state.active_spells.push_back(spell.clone());
                }
                q.push(state);
            }
        } else {
            let mut state = state.clone();
            state.player.hit_points -= i64::max(state.boss.damage - player_armor, 1);
            q.push(state);
        }

        q.sort_by(|a, b| {
            let a_dmg_per_mana = a.damange_dealt / a.mana_spent;
            let b_dmg_per_mana = b.damange_dealt / b.mana_spent;
            a_dmg_per_mana.cmp(&b_dmg_per_mana)
        });
    }

    best
}

#[cfg(test)]
mod tests {
    use crate::{
        initialize_battle, initialize_chars, initialize_spells,
        win_with_least_amount_of_mana_spent, Char,
    };

    #[test]
    fn test_examples() {
        let player = Char {
            hit_points: 10,
            mana: 250,
            damage: 0,
        };
        let boss = Char {
            hit_points: 13,
            mana: 0,
            damage: 8,
        };
        let state = initialize_battle(player, boss);
        let spells = initialize_spells();
        let result = win_with_least_amount_of_mana_spent(state, &spells, false);

        assert_eq!(result.player_won, true);
        assert_eq!(result.turns, 4);
        assert_eq!(result.mana_spent, 226);

        let player = Char {
            hit_points: 10,
            mana: 250,
            damage: 0,
        };
        let boss = Char {
            hit_points: 14,
            mana: 0,
            damage: 8,
        };
        let state = initialize_battle(player, boss);
        let result = win_with_least_amount_of_mana_spent(state, &spells, false);

        assert_eq!(result.player_won, true);
        assert_eq!(result.turns, 10);
        assert_eq!(result.mana_spent, 641);
    }

    #[test]
    fn test_input() {
        let (player, boss) = initialize_chars("input/boss.txt", 50, 500);
        let spells = initialize_spells();
        let state = initialize_battle(player, boss);

        let result = win_with_least_amount_of_mana_spent(state.clone(), &spells, false);

        assert_eq!(result.player_won, true);
        assert_eq!(result.turns, 18);
        assert_eq!(result.mana_spent, 953);

        let result = win_with_least_amount_of_mana_spent(state.clone(), &spells, true);

        assert_eq!(result.player_won, true);
        assert_eq!(result.turns, 18);
        assert_eq!(result.mana_spent, 1289);
    }
}
