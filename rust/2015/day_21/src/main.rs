#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2015 - day 21");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    t: ItemType,
    name: &'static str,
    cost: i64,
    dmg: i64,
    armor: i64,
}

impl Item {
    fn new(t: ItemType, name: &'static str, cost: i64, dmg: i64, armor: i64) -> Self {
        Self {
            t,
            name,
            cost,
            dmg,
            armor,
        }
    }
}

fn item_shop_inventory() -> Vec<Item> {
    vec![
        Item::new(ItemType::Weapon, "Dagger", 8, 4, 0),
        Item::new(ItemType::Weapon, "Shortsword", 10, 5, 0),
        Item::new(ItemType::Weapon, "Warhammer", 25, 6, 0),
        Item::new(ItemType::Weapon, "Longsword", 40, 7, 0),
        Item::new(ItemType::Weapon, "Greataxe", 74, 8, 0),
        Item::new(ItemType::Armor, "No Armor", 0, 0, 0),
        Item::new(ItemType::Armor, "Leather", 13, 0, 1),
        Item::new(ItemType::Armor, "Chainmail", 31, 0, 2),
        Item::new(ItemType::Armor, "Splintmail", 53, 0, 3),
        Item::new(ItemType::Armor, "Bandedmail", 75, 0, 4),
        Item::new(ItemType::Armor, "Platemail", 102, 0, 5),
        Item::new(ItemType::Ring, "No Ring (i)", 0, 0, 0),
        Item::new(ItemType::Ring, "No Ring (ii)", 0, 0, 0),
        Item::new(ItemType::Ring, "Damage +1", 25, 1, 0),
        Item::new(ItemType::Ring, "Damage +2", 50, 2, 0),
        Item::new(ItemType::Ring, "Damage +3", 100, 3, 0),
        Item::new(ItemType::Ring, "Defense +1", 20, 0, 1),
        Item::new(ItemType::Ring, "Defense +2", 40, 0, 2),
        Item::new(ItemType::Ring, "Defense +3", 80, 0, 3),
    ]
}

#[derive(Debug)]
struct Build {
    weapon: Item,
    armor: Item,
    ring_1: Item,
    ring_2: Item,
    cost: i64,
    attack: i64,
    defense: i64,
}

impl Build {
    fn new(weapon: Item, armor: Item, ring_1: Item, ring_2: Item) -> Self {
        if weapon.t != ItemType::Weapon
            || armor.t != ItemType::Armor
            || ring_1.t != ItemType::Ring
            || ring_2.t != ItemType::Ring
        {
            panic!("Illegal build!")
        }

        let attack = weapon.dmg + armor.dmg + ring_1.dmg + ring_2.dmg;
        let defense = weapon.armor + armor.armor + ring_1.armor + ring_2.armor;
        let cost = weapon.cost + armor.cost + ring_1.cost + ring_2.cost;

        Self {
            weapon,
            armor,
            ring_1,
            ring_2,
            cost,
            attack,
            defense,
        }
    }
}

fn create_all_builds() -> Vec<Build> {
    let mut result = Vec::new();

    let items = item_shop_inventory();
    let weapons: Vec<Item> = items
        .iter()
        .filter(|i| i.t == ItemType::Weapon)
        .map(|i| *i)
        .collect();
    let armor: Vec<Item> = items
        .iter()
        .filter(|i| i.t == ItemType::Armor)
        .map(|i| *i)
        .collect();
    let rings: Vec<Item> = items
        .iter()
        .filter(|i| i.t == ItemType::Ring)
        .map(|i| *i)
        .collect();

    for w in weapons.iter() {
        for a in armor.iter() {
            for (r1_idx, ring_1) in rings.iter().enumerate() {
                let rings_2 = rings
                    .iter()
                    .enumerate()
                    .filter(|(r2_idx, _)| *r2_idx > r1_idx);
                for (_, ring_2) in rings_2 {
                    result.push(Build::new(*w, *a, *ring_1, *ring_2));
                }
            }
        }
    }

    result
}

fn can_player_win(
    (mut p_hp, p_dmg, p_arm): (i64, i64, i64),
    (mut b_hp, b_dmg, b_arm): (i64, i64, i64),
) -> bool {
    let mut player_turn = true;
    while p_hp > 0 && b_hp > 0 {
        if player_turn {
            b_hp -= i64::max(p_dmg - b_arm, 1);
        } else {
            p_hp -= i64::max(b_dmg - p_arm, 1);
        }
        player_turn = !player_turn;
    }

    p_hp > 0
}

fn parse_boss(file: &str) -> (i64, i64, i64) {
    let mut hp = 0;
    let mut dmg = 0;
    let mut armor = 0;

    let file = std::fs::read_to_string(file).unwrap();
    for line in file.lines() {
        let mut split = line.split(':');
        match split.next().unwrap().trim() {
            "Hit Points" => hp = split.next().unwrap().trim().parse().unwrap(),
            "Damage" => dmg = split.next().unwrap().trim().parse().unwrap(),
            "Armor" => armor = split.next().unwrap().trim().parse().unwrap(),
            _ => panic!("illegal boss input"),
        }
    }

    (hp, dmg, armor)
}

#[cfg(test)]
mod tests {
    use crate::{can_player_win, create_all_builds, parse_boss};

    #[test]
    fn test_examples() {
        let player = (8, 5, 5);
        let boss = (12, 7, 2);

        assert_eq!(can_player_win(player, boss), true);
    }

    #[test]
    fn test_input() {
        let mut builds = create_all_builds();
        let (b_hp, b_dmg, b_arm) = parse_boss("input/boss.txt");

        assert_eq!(b_hp, 109);
        assert_eq!(b_dmg, 8);
        assert_eq!(b_arm, 2);

        builds.sort_by(|a, b| a.cost.cmp(&b.cost));
        let mut cheapest_winning_build = i64::MAX;
        for build in builds.iter() {
            if can_player_win((100, build.attack, build.defense), (b_hp, b_dmg, b_arm)) {
                cheapest_winning_build = build.cost;
                break;
            }
        }
        assert_eq!(cheapest_winning_build, 111);

        builds.reverse();
        let mut most_expensive_losing_build = i64::MIN;
        for build in builds.iter() {
            if !can_player_win((100, build.attack, build.defense), (b_hp, b_dmg, b_arm)) {
                most_expensive_losing_build = build.cost;
                break;
            }
        }

        assert_eq!(most_expensive_losing_build, 188);
    }
}
