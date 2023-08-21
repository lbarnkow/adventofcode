#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Advent of Code 2017 - day 24");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Component {
    a: usize,
    b: usize,
}

impl From<&str> for Component {
    fn from(value: &str) -> Self {
        let mut split = value.split("/");

        let a = split.next().unwrap().parse::<usize>().unwrap();
        let b = split.next().unwrap().parse::<usize>().unwrap();
        assert_eq!(split.next(), None);

        Self::new(a, b)
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.a, self.b)
    }
}

impl Component {
    fn new(a: usize, b: usize) -> Self {
        Self {
            a: a.min(b),
            b: a.max(b),
        }
    }

    fn from_listing(listing: &str) -> Vec<Self> {
        listing.lines().map(|line| line.into()).collect()
    }

    fn strength(&self) -> usize {
        self.a + self.b
    }
}

fn filter_possible_components(port: usize, components: &HashSet<Component>) -> Vec<Component> {
    components
        .iter()
        .filter(|c| c.a == port || c.b == port)
        .map(|c| *c)
        .collect()
}

fn find_best_bridge_go<Scorer, Score>(
    bridge: &mut Vec<Component>,
    scorer_fn: &Scorer,
    port: usize,
    components: &mut HashSet<Component>,
) -> Score
where
    Scorer: Fn(&Vec<Component>) -> Score,
    Score: Ord,
{
    let next_components = filter_possible_components(port, components);
    let mut strength = scorer_fn(bridge);

    for component in next_components {
        let next_port = if port == component.a {
            component.b
        } else {
            component.a
        };
        bridge.push(component);
        components.remove(&component);
        strength = strength.max(find_best_bridge_go(
            bridge, scorer_fn, next_port, components,
        ));
        bridge.pop();
        components.insert(component);
    }

    strength
}

fn find_best_bridge<Scorer, Score>(components: &mut HashSet<Component>, scorer_fn: &Scorer) -> Score
where
    Scorer: Fn(&Vec<Component>) -> Score,
    Score: Ord,
{
    find_best_bridge_go(&mut Vec::new(), scorer_fn, 0, components)
}

#[cfg(test)]
mod tests {
    use crate::{find_best_bridge, Component};
    use std::collections::HashSet;

    fn bridge_strength(bridge: &Vec<Component>) -> usize {
        bridge.iter().map(|c| c.strength()).sum()
    }

    #[test]
    fn test_examples() {
        let listing = "\
            0/2\n\
            2/2\n\
            2/3\n\
            3/4\n\
            3/5\n\
            0/1\n\
            10/1\n\
            9/10\
        ";

        let components = Component::from_listing(listing);
        let mut components_set: HashSet<Component> =
            HashSet::from_iter(components.iter().map(|c| *c));

        assert_eq!(components.len(), components_set.len());

        let strength = find_best_bridge(&mut components_set, &bridge_strength);
        assert_eq!(strength, 31);
    }

    #[test]
    fn test_input() {
        let listing = std::fs::read_to_string("input/parts.txt").unwrap();
        let components = Component::from_listing(listing.as_str());
        let mut components_set: HashSet<Component> =
            HashSet::from_iter(components.iter().map(|c| *c));

        assert_eq!(components.len(), components_set.len());

        let strength = find_best_bridge(&mut components_set, &bridge_strength);
        assert_eq!(strength, 1511);
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct BridgeLenAndStrengthScore(usize, usize);

    #[test]
    fn test_examples_part2() {
        let listing = "\
            0/2\n\
            2/2\n\
            2/3\n\
            3/4\n\
            3/5\n\
            0/1\n\
            10/1\n\
            9/10\
        ";

        let components = Component::from_listing(listing);
        let mut components_set: HashSet<Component> =
            HashSet::from_iter(components.iter().map(|c| *c));

        assert_eq!(components.len(), components_set.len());

        let scorer_fn = |b: &Vec<Component>| BridgeLenAndStrengthScore(b.len(), bridge_strength(b));

        let strength = find_best_bridge(&mut components_set, &scorer_fn);
        assert_eq!(strength.1, 19);
    }

    #[test]
    fn test_input_part_2() {
        let listing = std::fs::read_to_string("input/parts.txt").unwrap();
        let components = Component::from_listing(listing.as_str());
        let mut components_set: HashSet<Component> =
            HashSet::from_iter(components.iter().map(|c| *c));

        assert_eq!(components.len(), components_set.len());

        let scorer_fn = |b: &Vec<Component>| BridgeLenAndStrengthScore(b.len(), bridge_strength(b));

        let strength = find_best_bridge(&mut components_set, &scorer_fn);
        assert_eq!(strength.1, 1471);
    }
}
