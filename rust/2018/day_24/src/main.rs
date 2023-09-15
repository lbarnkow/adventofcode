#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::BTreeMap, fmt::Display};

fn main() {
    println!("Advent of Code 2018 - day 24");
}

lazy_static! {
    static ref RE_GROUP: Regex = Regex::new(r"^(\d+) units each with (\d+) hit points ?(?:\((.+)\))? ?with an attack that does (\d+) (\w+) damage at initiative (\d+)$").unwrap();
    static ref RE_DEBUFF: Regex = Regex::new(r"^(\w+) to ([\w, ]+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DamageType {
    Cold,
    Slashing,
    Bludgeoning,
    Radiation,
    Fire,
}

impl From<&str> for DamageType {
    fn from(value: &str) -> Self {
        match value {
            "cold" => Self::Cold,
            "slashing" => Self::Slashing,
            "bludgeoning" => Self::Bludgeoning,
            "radiation" => Self::Radiation,
            "fire" => Self::Fire,
            _ => panic!("Illegal damage type: {value}!"),
        }
    }
}

impl Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DamageType::Cold => "cold",
            DamageType::Slashing => "slashing",
            DamageType::Bludgeoning => "bludgeoning",
            DamageType::Radiation => "radiation",
            DamageType::Fire => "fire",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BuffType {
    Weak,
    Immune,
}

impl From<&str> for BuffType {
    fn from(value: &str) -> Self {
        match value {
            "weak" => Self::Weak,
            "immune" => Self::Immune,
            _ => panic!("Illegal (de-)buff type: {value}!"),
        }
    }
}

impl Display for BuffType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BuffType::Weak => "weak",
            BuffType::Immune => "immune",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Buff {
    buff_type: BuffType,
    damage_type: DamageType,
}

impl Display for Buff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.buff_type, self.damage_type)
    }
}

impl Buff {
    fn from_multiple(value: &str) -> Vec<Self> {
        let mut buffs = Vec::new();

        for debuff in value.split("; ") {
            let caps = RE_DEBUFF.captures(debuff).unwrap();
            let buff_type = caps[1].into();
            for dt in caps[2].split(", ") {
                let damage_type = dt.into();
                buffs.push(Self {
                    buff_type,
                    damage_type,
                })
            }
        }

        buffs
    }
}

#[derive(Debug, Clone)]
struct Group {
    id: usize,
    faction: String,
    units: usize,
    hit_points: usize,
    buffs: Vec<Buff>,
    attack: usize,
    attack_type: DamageType,
    initiative: usize,
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} units each with {} hit points ",
            self.units, self.hit_points
        )
        .unwrap();

        if !self.buffs.is_empty() {
            let mut sep = "";
            let mut s = String::new();
            for buff in &self.buffs {
                s.push_str(sep);
                s.push_str(&buff.to_string());
                sep = "; ";
            }
            write!(f, "({}) ", s).unwrap();
        }

        write!(
            f,
            "with an attack that does {} {} damage at initiative {}",
            self.attack, self.attack_type, self.initiative
        )
    }
}

impl Group {
    fn from_str(value: &str, id: usize, faction: &str) -> Self {
        let caps = RE_GROUP.captures(value).unwrap();

        let faction = faction.to_string();
        let units = caps[1].parse().unwrap();
        let hit_points = caps[2].parse().unwrap();
        let buffs = if let Some(debuffs) = caps.get(3) {
            Buff::from_multiple(debuffs.as_str())
        } else {
            Vec::with_capacity(0)
        };
        let attack = caps[4].parse().unwrap();
        let attack_type = caps[5].into();
        let initiative = caps[6].parse().unwrap();

        Self {
            id,
            faction,
            units,
            hit_points,
            buffs,
            attack,
            attack_type,
            initiative,
        }
    }

    fn effective_power(&self) -> usize {
        self.units * self.attack
    }

    fn take_damage(&mut self, dmg: usize) -> usize {
        let units_before = self.units;

        let mut hp = units_before * self.hit_points;
        if dmg > hp {
            hp = 0;
        } else {
            hp -= dmg;
        }

        let mut units_left = hp / self.hit_points;
        if hp % self.hit_points > 0 {
            units_left += 1;
        }
        self.units = units_left;

        units_before - units_left
    }

    fn potential_damage_to(&self, group: &Group) -> usize {
        let damage = self.effective_power();

        group
            .buffs
            .iter()
            .filter(|buff| buff.damage_type == self.attack_type)
            .fold(damage, |acc, buff| match buff.buff_type {
                BuffType::Weak => acc * 2,
                BuffType::Immune => 0,
            })
    }

    fn select_target(&self, def_groups: &[&Group]) -> Option<usize> {
        def_groups
            .iter()
            .fold(
                (None, 0, 0, 0),
                |(target, best_dmg, best_ep, best_ini), def_group| {
                    let dmg = self.potential_damage_to(def_group);
                    let ep = def_group.effective_power();
                    let ini = def_group.initiative;
                    // println!(
                    //     "{} group {} would deal defending group {} {} damage",
                    //     self.faction, self.id, def_group.id, dmg
                    // );
                    let mut better = false;
                    if dmg == 0 {
                    } else if dmg > best_dmg
                        || (dmg == best_dmg && (ep > best_ep || (ep == best_ep && ini > best_ini)))
                    {
                        better = true;
                    }
                    if better {
                        (Some(def_group.id), dmg, ep, ini)
                    } else {
                        (target, best_dmg, best_ep, best_ini)
                    }
                },
            )
            .0
    }

    fn is_alive(&self) -> bool {
        self.units > 0
    }
}

#[derive(Debug, Clone)]
struct Army {
    id: usize,
    name: String,
    groups: BTreeMap<usize, Group>,
}

impl Display for Army {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.name).unwrap();
        for group in self.groups.values() {
            write!(f, "\n{}", group).unwrap();
        }
        Ok(())
    }
}

impl Army {
    fn from_str(value: &str, id: usize) -> Self {
        let mut iter = value.lines();
        let name = iter.next().unwrap().replace(':', "");
        let groups = iter
            .enumerate()
            .map(|(idx, line)| (idx + 1, Group::from_str(line, idx + 1, name.as_str())))
            .collect();

        Self { id, name, groups }
    }

    fn select_targets(&self, target: &Army) -> Vec<TargetMapping> {
        let mut atk_groups: Vec<&Group> = self.groups.values().collect();
        atk_groups.sort_by(|g1, g2| {
            match g1.effective_power().cmp(&g2.effective_power()) {
                std::cmp::Ordering::Equal => g1.initiative.cmp(&g2.initiative),
                cmp => cmp,
            }
            .reverse()
        });

        let mut target_mappings = Vec::new();

        let mut def_groups: Vec<&Group> = target.groups.values().collect();
        for atk_group in atk_groups {
            if let Some(def_group_id) = atk_group.select_target(&def_groups) {
                target_mappings.push(TargetMapping::new(
                    self.id,
                    atk_group.id,
                    target.id,
                    def_group_id,
                ));
                let (idx, _) = def_groups
                    .iter()
                    .enumerate()
                    .find(|(_, g)| g.id == def_group_id)
                    .unwrap();
                def_groups.remove(idx);
            }
        }

        target_mappings
    }

    fn is_alive(&self) -> bool {
        self.groups.iter().filter(|(_, g)| g.is_alive()).count() > 0
    }

    fn total_units(&self) -> usize {
        self.groups.values().map(|g| g.units).sum()
    }

    fn boost(&mut self, boost: usize) {
        for (_, group) in self.groups.iter_mut() {
            group.attack += boost;
        }
    }
}

#[derive(Debug, Clone)]
struct War {
    armies: BTreeMap<usize, Army>,
}

impl Display for War {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = "";
        for army in self.armies.values() {
            write!(f, "{}{}", sep, army).unwrap();
            sep = "\n\n";
        }
        Ok(())
    }
}

impl From<&str> for War {
    fn from(value: &str) -> Self {
        let iter = value.split("\n\n");

        let armies: BTreeMap<usize, Army> = iter
            .enumerate()
            .map(|(idx, army)| (idx + 1, Army::from_str(army, idx + 1)))
            .collect();

        assert_eq!(armies.len(), 2);

        Self { armies }
    }
}

impl War {
    fn boost_army(&mut self, name: &str, boost: usize) {
        for (_, army) in self.armies.iter_mut() {
            if army.name == name {
                army.boost(boost);
                return;
            }
        }
        panic!("No army named: {name}!");
    }

    fn target_phase(&self) -> Vec<TargetMapping> {
        let mut target_mappings = Vec::new();

        for (_, atk_army) in self.armies.iter().rev() {
            let def_army_id = if atk_army.id == 1 { 2 } else { 1 };
            let def_army = self.armies.get(&def_army_id).unwrap();
            target_mappings.append(&mut atk_army.select_targets(def_army));
        }
        // println!();

        target_mappings
    }

    fn attack_phase(&mut self, target_mappings: &mut [TargetMapping]) {
        target_mappings.sort_by(|m1, m2| {
            let grp1 = self
                .armies
                .get(&m1.atk_army)
                .unwrap()
                .groups
                .get(&m1.atk_group)
                .unwrap();
            let grp2 = self
                .armies
                .get(&m2.atk_army)
                .unwrap()
                .groups
                .get(&m2.atk_group)
                .unwrap();

            grp1.initiative.cmp(&grp2.initiative).reverse()
        });

        for target_mapping in target_mappings.iter() {
            let dmg = {
                let atk = self
                    .armies
                    .get(&target_mapping.atk_army)
                    .unwrap()
                    .groups
                    .get(&target_mapping.atk_group)
                    .unwrap();
                if atk.units == 0 {
                    continue;
                }
                let def = self
                    .armies
                    .get(&target_mapping.def_army)
                    .unwrap()
                    .groups
                    .get(&target_mapping.def_group)
                    .unwrap();
                if def.units == 0 {
                    continue;
                }

                // print!(
                //     "{} group {} attacks defending group {}, ",
                //     atk.faction, atk.id, def.id
                // );
                atk.potential_damage_to(def)
            };
            let def = self
                .armies
                .get_mut(&target_mapping.def_army)
                .unwrap()
                .groups
                .get_mut(&target_mapping.def_group)
                .unwrap();
            let _killed = def.take_damage(dmg);
            // println!("killing {} units", killed);
        }
    }

    fn cleanup_defeated(&mut self) {
        for (_, army) in self.armies.iter_mut() {
            let group_ids: Vec<usize> = army.groups.keys().copied().collect();
            for group_id in group_ids {
                if let Some(group) = army.groups.get(&group_id) {
                    if group.units == 0 {
                        army.groups.remove(&group_id);
                    }
                }
            }
        }
    }

    fn print_stats(&self) {
        for army in self.armies.values() {
            println!("{}:", army.name);
            for group in army.groups.values() {
                println!("Group {} contains {} units", group.id, group.units);
            }
            if army.groups.is_empty() {
                println!("No groups remain.");
            }
        }
    }

    fn fight(&mut self) {
        // self.print_stats();
        // println!();

        let mut target_mappings = self.target_phase();
        self.attack_phase(&mut target_mappings);
        // println!();

        self.cleanup_defeated();
    }

    fn fight_to_end(&mut self) -> WarOutcome {
        let mut units = self.total_units();

        while self.armies.values().filter(|a| a.is_alive()).count() > 1 {
            self.fight();
            // println!();

            let units_after = self.total_units();
            if units == units_after {
                return WarOutcome::Stalemate;
            }
            units = units_after;
        }

        // self.print_stats();
        WarOutcome::OneSideWon
    }

    fn total_units(&self) -> usize {
        self.armies.values().map(|a| a.total_units()).sum()
    }

    fn winning_team(&self) -> &str {
        for army in self.armies.values() {
            if army.total_units() > 0 {
                return army.name.as_str();
            }
        }
        panic!("All armies are dead?!");
    }
}

#[derive(Debug, PartialEq)]
enum WarOutcome {
    OneSideWon,
    Stalemate,
}

struct TargetMapping {
    atk_army: usize,
    atk_group: usize,
    def_army: usize,
    def_group: usize,
}

impl TargetMapping {
    fn new(atk_army: usize, atk_group: usize, def_army: usize, def_group: usize) -> Self {
        Self {
            atk_army,
            atk_group,
            def_army,
            def_group,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{War, WarOutcome};

    #[test]
    fn test_example() {
        let input = "\
            Immune System:\n\
            17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2\n\
            989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3\n\
            \n\
            Infection:\n\
            801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1\n\
            4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4\
        ";

        let mut war = War::from(input);

        let result = war.fight_to_end();
        assert_eq!(war.total_units(), 5216);
        assert_eq!(war.winning_team(), "Infection");
        assert_eq!(result, WarOutcome::OneSideWon);

        let war = War::from(input);
        let mut boost = 0;
        let mut units_left = 0;
        for i in 0..10000 {
            let mut war = war.clone();
            boost = i;
            war.boost_army("Immune System", boost);
            let result = war.fight_to_end();
            if result == WarOutcome::OneSideWon && war.winning_team() == "Immune System" {
                units_left = war.total_units();
                break;
            }
        }
        assert_eq!(boost, 1570);
        assert_eq!(units_left, 51);
    }

    #[test]
    fn test_example_2() {
        // this reddit example helped me find a bug in target selection:
        // https://www.reddit.com/r/adventofcode/comments/a91ysq/2018_day_24_solutions/eckknx0/
        let input = "\
            Immune System:\n\
            100 units each with 10 hit points with an attack that does 100 slashing damage at initiative 3\n\
            99 units each with 9 hit points (weak to radiation) with an attack that does 99 fire damage at initiative 2\n\
            \n\
            Infection:\n\
            2 units each with 2 hit points (immune to slashing) with an attack that does 900 radiation damage at initiative 1\
        ";

        let mut war = War::from(input);

        let result = war.fight_to_end();
        assert_eq!(war.total_units(), 199);
        assert_eq!(war.winning_team(), "Immune System");
        assert_eq!(result, WarOutcome::OneSideWon);

        let war = War::from(input);
        let mut boost = 0;
        let mut units_left = 0;
        for i in 0..10000 {
            let mut war = war.clone();
            boost = i;
            war.boost_army("Immune System", boost);
            let result = war.fight_to_end();
            if result == WarOutcome::OneSideWon && war.winning_team() == "Immune System" {
                units_left = war.total_units();
                break;
            }
        }
        assert_eq!(boost, 0);
        assert_eq!(units_left, 199);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/armies.txt").unwrap();
        let mut war = War::from(input.as_str());

        let result = war.fight_to_end();
        assert_eq!(war.total_units(), 18280);
        assert_eq!(war.winning_team(), "Infection");
        assert_eq!(result, WarOutcome::OneSideWon);

        let war = War::from(input.as_str());
        let mut boost = 0;
        let mut units_left = 0;
        for i in 0..10000 {
            let mut war = war.clone();
            boost = i;
            war.boost_army("Immune System", boost);
            let result = war.fight_to_end();
            if result == WarOutcome::OneSideWon && war.winning_team() == "Immune System" {
                units_left = war.total_units();
                break;
            }
        }
        assert_eq!(boost, 28);
        assert_eq!(units_left, 4573);
    }
}
