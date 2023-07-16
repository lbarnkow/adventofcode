#![allow(dead_code)]
use regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 16");
}

#[derive(Debug, Default)]
struct Aunt {
    number: u64,
    props: Props,
}

impl Aunt {
    fn new(number: u64, props: Props) -> Self {
        Self { number, props }
    }

    fn parse(file: &str) -> Vec<Aunt> {
        let re = Regex::new(r"^Sue (\d+): (.+)$").unwrap();
        let mut result = Vec::new();

        let props_file = std::fs::read_to_string(file).unwrap();
        for line in props_file.lines() {
            let caps = re.captures(&line).unwrap();
            let number: u64 = caps[1].parse().unwrap();
            let props = Props::parse(&caps[2]);
            result.push(Aunt::new(number, props));
        }

        result
    }
}

#[derive(Debug, Default)]
struct Props {
    children: Option<u64>,
    cats: Option<u64>,
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

impl Props {
    fn parse_prop(&mut self, prop: &str) -> &Self {
        let prop: Vec<&str> = prop.trim().split(':').collect();

        match prop[0].trim() {
            "children" => self.children = Some(prop[1].trim().parse().unwrap()),
            "cats" => self.cats = Some(prop[1].trim().parse().unwrap()),
            "samoyeds" => self.samoyeds = Some(prop[1].trim().parse().unwrap()),
            "pomeranians" => self.pomeranians = Some(prop[1].trim().parse().unwrap()),
            "akitas" => self.akitas = Some(prop[1].trim().parse().unwrap()),
            "vizslas" => self.vizslas = Some(prop[1].trim().parse().unwrap()),
            "goldfish" => self.goldfish = Some(prop[1].trim().parse().unwrap()),
            "trees" => self.trees = Some(prop[1].trim().parse().unwrap()),
            "cars" => self.cars = Some(prop[1].trim().parse().unwrap()),
            "perfumes" => self.perfumes = Some(prop[1].trim().parse().unwrap()),
            _ => panic!("Unsupported property!"),
        }

        self
    }

    fn parse(props: &str) -> Self {
        let mut result = Self::default();
        for prop in props.split(',') {
            result.parse_prop(prop);
        }
        result
    }

    fn prop_eq(a: &Option<u64>, b: &Option<u64>) -> bool {
        if a.is_none() || b.is_none() {
            true
        } else {
            a.unwrap() == b.unwrap()
        }
    }

    fn prop_gt(a: &Option<u64>, b: &Option<u64>) -> bool {
        if a.is_none() || b.is_none() {
            true
        } else {
            a.unwrap() > b.unwrap()
        }
    }

    fn prop_lt(a: &Option<u64>, b: &Option<u64>) -> bool {
        if a.is_none() || b.is_none() {
            true
        } else {
            a.unwrap() < b.unwrap()
        }
    }

    fn matches(&self, other: &Props) -> bool {
        Props::prop_eq(&self.children, &other.children)
            && Props::prop_eq(&self.cats, &other.cats)
            && Props::prop_eq(&self.samoyeds, &other.samoyeds)
            && Props::prop_eq(&self.pomeranians, &other.pomeranians)
            && Props::prop_eq(&self.akitas, &other.akitas)
            && Props::prop_eq(&self.vizslas, &other.vizslas)
            && Props::prop_eq(&self.goldfish, &other.goldfish)
            && Props::prop_eq(&self.trees, &other.trees)
            && Props::prop_eq(&self.cars, &other.cars)
            && Props::prop_eq(&self.perfumes, &other.perfumes)
    }

    fn matches_calibrated(&self, other: &Props) -> bool {
        Props::prop_eq(&self.children, &other.children)
            && Props::prop_lt(&self.cats, &other.cats)
            && Props::prop_eq(&self.samoyeds, &other.samoyeds)
            && Props::prop_gt(&self.pomeranians, &other.pomeranians)
            && Props::prop_eq(&self.akitas, &other.akitas)
            && Props::prop_eq(&self.vizslas, &other.vizslas)
            && Props::prop_gt(&self.goldfish, &other.goldfish)
            && Props::prop_lt(&self.trees, &other.trees)
            && Props::prop_eq(&self.cars, &other.cars)
            && Props::prop_eq(&self.perfumes, &other.perfumes)
    }
}

fn select_aunt(detected_props: &Props, aunts: &Vec<Aunt>) -> usize {
    let aunts: Vec<usize> = aunts
        .iter()
        .enumerate()
        .filter(|(_, aunt)| detected_props.matches(&aunt.props))
        .map(|(idx, _)| idx)
        .collect();

    assert_eq!(aunts.len(), 1);
    aunts[0]
}

fn select_aunt_calibrated(detected_props: &Props, aunts: &Vec<Aunt>) -> usize {
    let aunts: Vec<usize> = aunts
        .iter()
        .enumerate()
        .filter(|(_, aunt)| detected_props.matches_calibrated(&aunt.props))
        .map(|(idx, _)| idx)
        .collect();

    assert_eq!(aunts.len(), 1);
    aunts[0]
}

#[cfg(test)]
mod tests {
    use crate::{select_aunt, select_aunt_calibrated, Aunt, Props};

    #[test]
    fn test_input() {
        let props = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";
        let detected_props = Props::parse(props);

        let aunts = Aunt::parse("input/aunts.txt");

        let idx = select_aunt(&detected_props, &aunts);
        assert_eq!(aunts[idx].number, 40);

        let idx = select_aunt_calibrated(&detected_props, &aunts);
        assert_eq!(aunts[idx].number, 241);
    }
}
