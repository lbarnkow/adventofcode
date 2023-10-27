#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2019 - day 14");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Element(String);

#[derive(Debug, Clone)]
struct Chemical {
    element: Element,
    amount: usize,
}

impl Chemical {
    fn new(element: Element, amount: usize) -> Self {
        Self { element, amount }
    }
}

impl From<&str> for Chemical {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let amount = split.next().unwrap().parse().unwrap();
        let element = Element(split.next().unwrap().to_string());
        assert_eq!(split.next(), None);
        Self::new(element, amount)
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn needs(&self, element: &Element) -> bool {
        self.inputs
            .iter()
            .filter(|c| c.element == element.clone())
            .count()
            > 0
    }
}

impl From<&str> for Reaction {
    fn from(value: &str) -> Self {
        let mut split = value.split(" => ");
        let lhs = split.next().unwrap();
        let rhs = split.next().unwrap();
        assert_eq!(split.next(), None);

        let output = rhs.into();
        let inputs = lhs.split(", ").map(|c| c.into()).collect();

        Self { inputs, output }
    }
}

struct RevTable {
    deps: HashMap<Element, Reaction>,
    order: Vec<Element>,
}

impl From<&str> for RevTable {
    fn from(value: &str) -> Self {
        let deps: HashMap<Element, Reaction> = value
            .lines()
            .map(Reaction::from)
            .map(|r| (r.output.element.clone(), r))
            .collect();

        // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
        let mut graph = deps.clone();
        let mut order = Vec::with_capacity(deps.len());
        let mut open = vec![Element("ORE".to_string())];
        while let Some(node) = open.pop() {
            order.push(node.clone());
            for reaction in graph.iter_mut().map(|(_, r)| r).filter(|r| r.needs(&node)) {
                let idx = reaction
                    .inputs
                    .iter()
                    .enumerate()
                    .filter(|(_idx, c)| c.element == node)
                    .map(|(idx, _)| idx)
                    .next()
                    .unwrap();
                reaction.inputs.remove(idx);
                if reaction.inputs.is_empty() {
                    open.push(reaction.output.element.clone());
                }
            }
        }

        order.reverse();
        order.pop();

        Self { deps, order }
    }
}

fn min_ore_required(table: &RevTable, fuel_amount: usize) -> usize {
    let mut need = HashMap::new();
    need.insert(Element("FUEL".to_string()), fuel_amount);

    for element in &table.order {
        let reaction = table.deps.get(element).unwrap();
        let amount_needed = need.get(element).unwrap();
        let mut factor = amount_needed / reaction.output.amount;
        if amount_needed % reaction.output.amount != 0 {
            factor += 1;
        }
        for input in &reaction.inputs {
            if let Some(prev_amount) = need.get_mut(&input.element) {
                *prev_amount += input.amount * factor;
            } else {
                need.insert(input.element.clone(), input.amount * factor);
            }
        }
        need.remove(element);
    }

    assert_eq!(need.len(), 1);
    need.remove(&Element("ORE".to_string())).unwrap()
}

fn max_fuel(table: &RevTable, ore_amount: usize) -> usize {
    let (mut low, mut high) = (0, ore_amount * 10);

    while high - low > 1 {
        let mid = (low + high) / 2;
        let ore_needed_for_mid = min_ore_required(table, mid);

        println!("{low} --- {mid} --- {high} --- {ore_needed_for_mid}");

        if ore_needed_for_mid > ore_amount {
            high = mid;
        } else {
            low = mid;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use crate::{max_fuel, min_ore_required, RevTable};

    #[test]
    fn test_example_1() {
        let reactions = "\
            10 ORE => 10 A\n\
            1 ORE => 1 B\n\
            7 A, 1 B => 1 C\n\
            7 A, 1 C => 1 D\n\
            7 A, 1 D => 1 E\n\
            7 A, 1 E => 1 FUEL\
        ";

        let table = RevTable::from(reactions);

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 31);
    }

    #[test]
    fn test_example_2() {
        let reactions = "\
            9 ORE => 2 A\n\
            8 ORE => 3 B\n\
            7 ORE => 5 C\n\
            3 A, 4 B => 1 AB\n\
            5 B, 7 C => 1 BC\n\
            4 C, 1 A => 1 CA\n\
            2 AB, 3 BC, 4 CA => 1 FUEL\
        ";

        let table = RevTable::from(reactions);

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 165);
    }

    #[test]
    fn test_example_3() {
        let reactions = "\
            157 ORE => 5 NZVS\n\
            165 ORE => 6 DCFZ\n\
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
            179 ORE => 7 PSHF\n\
            177 ORE => 5 HKGWZ\n\
            7 DCFZ, 7 PSHF => 2 XJWVT\n\
            165 ORE => 2 GPVTF\n\
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\
        ";

        let table = RevTable::from(reactions);

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 13312);
        let max_fuel = max_fuel(&table, 1_000_000_000_000);
        assert_eq!(max_fuel, 82892753);
    }

    #[test]
    fn test_example_4() {
        let reactions = "\
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n\
            17 NVRVD, 3 JNWZP => 8 VPVL\n\
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n\
            22 VJHF, 37 MNCFX => 5 FWMGM\n\
            139 ORE => 4 NVRVD\n\
            144 ORE => 7 JNWZP\n\
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n\
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n\
            145 ORE => 6 MNCFX\n\
            1 NVRVD => 8 CXFTF\n\
            1 VJHF, 6 MNCFX => 4 RFSQX\n\
            176 ORE => 6 VJHF\
        ";

        let table = RevTable::from(reactions);

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 180697);
        let max_fuel = max_fuel(&table, 1_000_000_000_000);
        assert_eq!(max_fuel, 5586022);
    }

    #[test]
    fn test_example_5() {
        let reactions = "\
            171 ORE => 8 CNZTR\n\
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
            114 ORE => 4 BHXH\n\
            14 VRPVC => 6 BMBT\n\
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
            5 BMBT => 4 WPTQ\n\
            189 ORE => 9 KTJDG\n\
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
            12 VRPVC, 27 CNZTR => 2 XDBXC\n\
            15 KTJDG, 12 BHXH => 5 XCVML\n\
            3 BHXH, 2 VRPVC => 7 MZWV\n\
            121 ORE => 7 VRPVC\n\
            7 XCVML => 6 RJRHP\n\
            5 BHXH, 4 VRPVC => 5 LTCX\
        ";

        let table = RevTable::from(reactions);

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 2210736);
        let max_fuel = max_fuel(&table, 1_000_000_000_000);
        assert_eq!(max_fuel, 460664);
    }

    #[test]
    fn test_input() {
        let reactions = std::fs::read_to_string("input/reactions.txt").unwrap();

        let table = RevTable::from(reactions.as_str());

        let min_ore = min_ore_required(&table, 1);
        assert_eq!(min_ore, 158482);
        let max_fuel = max_fuel(&table, 1_000_000_000_000);
        assert_eq!(max_fuel, 7993831);
    }
}
