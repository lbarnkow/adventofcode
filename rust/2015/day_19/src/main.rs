#![allow(dead_code)]
use std::collections::{BinaryHeap, HashSet};

fn main() {
    println!("Advent of Code 2015 - day 19");
}

fn parse_replacements(replacements: &str) -> HashSet<(String, String)> {
    replacements
        .lines()
        .map(|line| line.split("=>"))
        .map(|mut split| {
            (
                split.next().unwrap().trim().to_owned(),
                split.next().unwrap().trim().to_owned(),
            )
        })
        .collect()
}

fn parse_input_file(file: &str) -> (String, HashSet<(String, String)>) {
    let file = std::fs::read_to_string(file).unwrap();
    let mut lines = file.lines().rev();

    let target = lines.next().unwrap();
    assert!(lines.next().unwrap().trim().is_empty());

    let lines = lines.collect::<Vec<&str>>().join("\n");
    let replacements = parse_replacements(&lines);

    (target.to_owned(), replacements)
}

fn apply_replacement(s: &str, needle: &str, replacement: &str) -> Vec<String> {
    let mut tmp = s;
    let mut result = Vec::new();

    while let Some(idx) = tmp.find(needle) {
        let mut new = String::new();
        let start = s.len() - tmp.len() + idx;
        new.push_str(&s[..start]);
        new.push_str(replacement);
        new.push_str(&s[start + needle.len()..]);
        result.push(new);
        tmp = &tmp[idx + needle.len()..];
    }

    result
}

fn get_unique_transformations(
    molecule: &str,
    replacements: &HashSet<(String, String)>,
) -> HashSet<String> {
    replacements
        .iter()
        .map(|(lhs, rhs)| apply_replacement(molecule, lhs, rhs))
        .flatten()
        .collect()
}

#[derive(Debug, Eq)]
struct Molecule {
    state: String,
    steps: usize,
}

impl Molecule {
    fn new(state: String, steps: usize) -> Self {
        Self { state, steps }
    }
}

impl PartialEq for Molecule {
    fn eq(&self, other: &Self) -> bool {
        self.state.len() == other.state.len() && self.steps == other.steps
    }
}

impl PartialOrd for Molecule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = self.state.len().cmp(&other.state.len());
        if ord.is_eq() {
            Some(self.steps.cmp(&other.steps).reverse())
        } else {
            Some(ord.reverse())
        }
    }
}

impl Ord for Molecule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.state.len().cmp(&other.state.len());
        ord
    }
}

fn get_shortest_transformation_step_count(
    target: &str,
    start: &str,
    replacements: &HashSet<(String, String)>,
) -> usize {
    // let mut q = VecDeque::new();
    let mut q = BinaryHeap::new();
    // q.push_back((target.to_owned(), 0));
    q.push(Molecule::new(target.to_owned(), 0));

    while let Some(molecule) = q.pop() {
        println!("{molecule:?}");
        let steps = molecule.steps + 1;
        //
        for (lhs, rhs) in replacements {
            for s in apply_replacement(&molecule.state, rhs, lhs) {
                if &s == start {
                    return steps;
                }
                q.push(Molecule::new(s, steps));
            }
        }
    }

    panic!("Failed");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        apply_replacement, get_shortest_transformation_step_count, get_unique_transformations,
        parse_input_file, parse_replacements,
    };

    #[test]
    fn test_examples() {
        let replacements = "\
            H => HO\n\
            H => OH\n\
            O => HH\
        ";

        let replacements = parse_replacements(replacements);
        assert_eq!(
            replacements,
            vec![
                ("H".to_owned(), "HO".to_owned()),
                ("H".to_owned(), "OH".to_owned()),
                ("O".to_owned(), "HH".to_owned())
            ]
            .into_iter()
            .collect()
        );

        let result = apply_replacement("ABC", "H", "HO");
        assert_eq!(result.len(), 0);
        let result = apply_replacement("HOH", "H", "HO");
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"HOOH".to_owned()));
        assert!(result.contains(&"HOHO".to_owned()));

        let start = "HOH";
        let result = get_unique_transformations(start, &replacements);
        assert_eq!(result.len(), 4);
        assert_eq!(
            result,
            vec!["HOOH", "HOHO", "OHOH", "HHHH"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>()
        );

        let start = "HOHOHO";
        let result = get_unique_transformations(start, &replacements);
        assert_eq!(result.len(), 7);
        assert_eq!(
            result,
            vec!["HOOHOHO", "HOHOOHO", "HOHOHOO", "OHOHOHO", "HHHHOHO", "HOHHHHO", "HOHOHHH",]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>()
        );

        // # part 2

        let replacements = "\
            e => H\n\
            e => O\n\
            H => HO\n\
            H => OH\n\
            O => HH\
        ";

        let replacements = parse_replacements(replacements);
        assert_eq!(
            replacements,
            vec![
                ("e".to_owned(), "H".to_owned()),
                ("e".to_owned(), "O".to_owned()),
                ("H".to_owned(), "HO".to_owned()),
                ("H".to_owned(), "OH".to_owned()),
                ("O".to_owned(), "HH".to_owned())
            ]
            .into_iter()
            .collect()
        );

        let start = "e";
        let target = "HOH";
        assert_eq!(
            get_shortest_transformation_step_count(target, start, &replacements),
            3
        );

        let start = "e";
        let target = "HOHOHO";
        assert_eq!(
            get_shortest_transformation_step_count(target, start, &replacements),
            6
        );
    }

    #[test]
    fn test_input() {
        let (target, replacements) = parse_input_file("input/molecules.txt");

        assert_eq!(target, "CRnCaCaCaSiRnBPTiMgArSiRnSiRnMgArSiRnCaFArTiTiBSiThFYCaFArCaCaSiThCaPBSiThSiThCaCaPTiRnPBSiThRnFArArCaCaSiThCaSiThSiRnMgArCaPTiBPRnFArSiThCaSiRnFArBCaSiRnCaPRnFArPMgYCaFArCaPTiTiTiBPBSiThCaPTiBPBSiRnFArBPBSiRnCaFArBPRnSiRnFArRnSiRnBFArCaFArCaCaCaSiThSiThCaCaPBPTiTiRnFArCaPTiBSiAlArPBCaCaCaCaCaSiRnMgArCaSiThFArThCaSiThCaSiRnCaFYCaSiRnFYFArFArCaSiRnFYFArCaSiRnBPMgArSiThPRnFArCaSiRnFArTiRnSiRnFYFArCaSiRnBFArCaSiRnTiMgArSiThCaSiThCaFArPRnFArSiRnFArTiTiTiTiBCaCaSiRnCaCaFYFArSiThCaPTiBPTiBCaSiThSiRnMgArCaF");
        assert_eq!(replacements.len(), 43);

        let result = get_unique_transformations(&target, &replacements);
        assert_eq!(result.len(), 535);

        // # part 2

        let start = "e";
        assert_eq!(
            get_shortest_transformation_step_count(&target, start, &replacements),
            212
        );
    }
}
