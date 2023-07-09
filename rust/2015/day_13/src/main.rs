#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;
fn main() {
    println!("Advent of Code 2015 - day 13");
}

fn parse_constraints(constraints: &str) -> (VecDeque<String>, HashMap<(String, String), i64>) {
    let re =
        Regex::new(r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$").unwrap();

    let mut people = HashSet::new();
    let mut relationships = HashMap::new();

    for line in constraints.lines() {
        let caps = re.captures(line).unwrap();

        let a = caps[1].to_owned();
        let b = caps[4].to_owned();
        let mut diff: i64 = caps[3].parse().unwrap();
        if &caps[2] == "lose" {
            diff = -diff;
        }

        people.insert(a.clone());
        people.insert(b.clone());
        relationships.insert((a, b), diff);
    }

    (people.into_iter().collect(), relationships)
}

fn compute_happiness(
    seated_people: &mut VecDeque<String>,
    relationships: &HashMap<(String, String), i64>,
) -> i64 {
    let mut happiness = 0;

    for i in 0..seated_people.len() {
        let middle = seated_people.get(i).unwrap();
        let left = if i > 0 {
            seated_people.get(i - 1).unwrap()
        } else {
            seated_people.get(seated_people.len() - 1).unwrap()
        };
        let right = if i < seated_people.len() - 1 {
            seated_people.get(i + 1).unwrap()
        } else {
            seated_people.get(0).unwrap()
        };

        happiness += relationships.get(&(middle.clone(), left.clone())).unwrap();
        happiness += relationships.get(&(middle.clone(), right.clone())).unwrap();
    }

    return happiness;
}

fn compute_max_happiness_2(
    seated_people: &mut VecDeque<String>,
    available_people: &mut VecDeque<String>,
    relationships: &HashMap<(String, String), i64>,
) -> i64 {
    if available_people.is_empty() {
        return compute_happiness(seated_people, relationships);
    }

    let mut max_happiness = i64::MIN;
    for _ in 0..available_people.len() {
        let person = available_people.pop_front().unwrap();
        seated_people.push_back(person);

        max_happiness = i64::max(
            max_happiness,
            compute_max_happiness_2(seated_people, available_people, relationships),
        );

        let person = seated_people.pop_back().unwrap();
        available_people.push_back(person);
    }

    max_happiness
}

fn compute_total_change_in_happiness(constraints: &str) -> i64 {
    let (mut people, relationships) = parse_constraints(constraints);
    compute_max_happiness_2(&mut VecDeque::new(), &mut people, &relationships)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{compute_happiness, compute_total_change_in_happiness, parse_constraints};

    #[test]
    fn test_examples() {
        let constraints = "Alice would gain 54 happiness units by sitting next to Bob.\n\
            Alice would lose 79 happiness units by sitting next to Carol.\n\
            Alice would lose 2 happiness units by sitting next to David.\n\
            Bob would gain 83 happiness units by sitting next to Alice.\n\
            Bob would lose 7 happiness units by sitting next to Carol.\n\
            Bob would lose 63 happiness units by sitting next to David.\n\
            Carol would lose 62 happiness units by sitting next to Alice.\n\
            Carol would gain 60 happiness units by sitting next to Bob.\n\
            Carol would gain 55 happiness units by sitting next to David.\n\
            David would gain 46 happiness units by sitting next to Alice.\n\
            David would lose 7 happiness units by sitting next to Bob.\n\
            David would gain 41 happiness units by sitting next to Carol.";

        let (_, relationships) = parse_constraints(constraints);
        let mut seated: VecDeque<String> = vec!["David", "Alice", "Bob", "Carol"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        assert_eq!(compute_happiness(&mut seated, &relationships), 330);

        assert_eq!(compute_total_change_in_happiness(constraints), 330);
    }

    #[test]
    fn test_input() {
        let constraints = std::fs::read_to_string("input/happiness.txt").unwrap();
        assert_eq!(compute_total_change_in_happiness(&constraints), 733);

        let constraints = std::fs::read_to_string("input/happiness_plus.txt").unwrap();
        assert_eq!(compute_total_change_in_happiness(&constraints), 725);
    }
}
