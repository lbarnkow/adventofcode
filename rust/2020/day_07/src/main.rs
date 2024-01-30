#![allow(dead_code)]

use std::{fmt::Display, hash::Hash, num::ParseIntError};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2020 - day 07");
}

#[derive(Debug)]
struct TryFromError {
    msg: String,
}

impl From<&str> for TryFromError {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl From<String> for TryFromError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERR: {}", &self.msg)
    }
}

impl From<ParseIntError> for TryFromError {
    fn from(value: ParseIntError) -> Self {
        value.to_string().into()
    }
}

lazy_static! {
    static ref RE_RULE: Regex = Regex::new(
        r"^(\w+\s\w+) bags contain ((?:no other bags)|(?:(?:, )?\d+\s\w+\s\w+\sbags?)+).$"
    )
    .unwrap();
    static ref RE_INNER_BAG: Regex = Regex::new(r"^(\d+)\s(\w+\s\w+)\sbags?$").unwrap();
}

type Color = String;

#[derive(Debug)]
struct Rule {
    outer: Color,
    inner: Vec<(usize, Color)>,
}

impl TryFrom<&str> for Rule {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let caps = RE_RULE
            .captures(value)
            .ok_or_else(|| -> TryFromError { format!("Illegal input: '{value}'!").into() })?;

        let outer = caps[1].to_owned();
        let inner = &caps[2];

        let inner = if inner == "no other bags" {
            Vec::with_capacity(0)
        } else {
            let mut v = Vec::new();
            for bag in inner.split(", ") {
                let caps = RE_INNER_BAG
                    .captures(bag)
                    .ok_or_else(|| -> TryFromError { format!("Illegal input: '{bag}'!").into() })?;
                let num = caps[1]
                    .parse::<usize>()
                    .map_err(|_| -> TryFromError { format!("Illegal input: '{bag}'!").into() })?;
                let color = caps[2].to_owned();
                v.push((num, color));
            }
            v
        };

        Ok(Self { outer, inner })
    }
}

impl Rule {
    fn try_from_many(raw_rules: &str) -> Result<Vec<Self>, TryFromError> {
        let mut rules = Vec::new();

        for line in raw_rules.lines() {
            rules.push(line.try_into()?);
        }

        Ok(rules)
    }

    fn contains(&self, color: &str) -> bool {
        self.inner.iter().any(|bag| bag.1 == color)
    }

    fn find_roots<'a>(&'a self, rules: &'a [Self]) -> Vec<&'a Self> {
        Self::find_roots_go(&mut Vec::new(), self, rules)
    }

    fn find_roots_go<'a>(
        visited: &mut Vec<&'a str>,
        current: &'a Self,
        rules: &'a [Self],
    ) -> Vec<&'a Self> {
        if visited.contains(&current.outer.as_str()) {
            return Vec::with_capacity(0);
        }
        visited.push(&current.outer);

        let mut roots = vec![current];

        rules
            .iter()
            .filter(|r| r.contains(&current.outer))
            .for_each(|r| {
                roots.append(&mut Self::find_roots_go(visited, r, rules));
            });

        if roots.is_empty() {
            vec![current]
        } else {
            roots
        }
    }

    fn find_rule<'a>(rules: &'a [Self], outer: &str) -> &'a Self {
        rules
            .iter()
            .find(|r| r.outer == outer)
            .expect("Could not find requested rule!")
    }

    fn count_children(&self, rules: &[Self]) -> usize {
        self.inner.iter().fold(0, |acc, (count, color)| {
            acc + count
                + count * Self::count_children(Self::find_rule(rules, color.as_str()), rules)
        })
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.outer == other.outer
    }
}

impl Eq for Rule {}

impl Hash for Rule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.outer.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{Rule, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let rules = "\
            light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
            bright white bags contain 1 shiny gold bag.\n\
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
            faded blue bags contain no other bags.\n\
            dotted black bags contain no other bags.\
        ";
        let rules = Rule::try_from_many(rules)?;

        let roots =
            rules
                .iter()
                .filter(|r| r.contains("shiny gold"))
                .fold(HashSet::new(), |mut acc, r| {
                    acc.extend(r.find_roots(&rules).into_iter());
                    acc
                });

        assert_eq!(roots.len(), 4);
        assert!(roots.iter().any(|r| r.outer == "bright white"));
        assert!(roots.iter().any(|r| r.outer == "muted yellow"));
        assert!(roots.iter().any(|r| r.outer == "light red"));
        assert!(roots.iter().any(|r| r.outer == "dark orange"));

        let shiny_gold_rule = Rule::find_rule(&rules, "shiny gold");
        let children = shiny_gold_rule.count_children(&rules);
        assert_eq!(children, 32);

        let rules = "\
            shiny gold bags contain 2 dark red bags.\n\
            dark red bags contain 2 dark orange bags.\n\
            dark orange bags contain 2 dark yellow bags.\n\
            dark yellow bags contain 2 dark green bags.\n\
            dark green bags contain 2 dark blue bags.\n\
            dark blue bags contain 2 dark violet bags.\n\
            dark violet bags contain no other bags.\
        ";
        let rules = Rule::try_from_many(rules)?;

        let shiny_gold_rule = Rule::find_rule(&rules, "shiny gold");
        let children = shiny_gold_rule.count_children(&rules);
        assert_eq!(children, 126);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let rules = std::fs::read_to_string("input/rules.txt").unwrap();
        let rules = Rule::try_from_many(&rules)?;

        let roots =
            rules
                .iter()
                .filter(|r| r.contains("shiny gold"))
                .fold(HashSet::new(), |mut acc, r| {
                    acc.extend(r.find_roots(&rules).into_iter());
                    acc
                });

        assert_eq!(roots.len(), 257);

        let shiny_gold_rule = Rule::find_rule(&rules, "shiny gold");
        let children = shiny_gold_rule.count_children(&rules);
        assert_eq!(children, 1038);

        Ok(())
    }
}
