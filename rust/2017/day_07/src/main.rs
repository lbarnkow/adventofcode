#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Advent of Code 2017 - day 07");
}

lazy_static! {
    static ref RE_NODE: Regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> ([\w, ]+))?$").unwrap();
}

#[derive(Debug)]
enum Node {
    RawBranch {
        name: String,
        weight: usize,
        children: Vec<String>,
    },
    Branch {
        name: String,
        weight: usize,
        children: Vec<Node>,
    },
    Leaf {
        name: String,
        weight: usize,
    },
}

impl Node {
    fn from_single(value: &str) -> Self {
        let caps = RE_NODE.captures(value).unwrap();

        let name = caps[1].to_owned();
        let weight = caps[2].parse().unwrap();

        if let Some(children) = caps.get(3) {
            let children = children
                .as_str()
                .split(", ")
                .map(|s| s.to_owned())
                .collect();
            Self::RawBranch {
                name,
                weight,
                children,
            }
        } else {
            Self::Leaf { name, weight }
        }
    }

    fn name(&self) -> &str {
        match self {
            Node::RawBranch {
                name,
                weight: _,
                children: _,
            } => name.as_str(),
            Node::Branch {
                name,
                weight: _,
                children: _,
            } => name.as_str(),
            Node::Leaf { name, weight: _ } => name.as_str(),
        }
    }

    fn weight(&self) -> usize {
        match self {
            Node::RawBranch {
                name: _,
                weight,
                children: _,
            } => *weight,
            Node::Branch {
                name: _,
                weight,
                children,
            } => *weight + children.iter().map(|child| child.weight()).sum::<usize>(),
            Node::Leaf { name: _, weight } => *weight,
        }
    }

    fn self_weight(&self) -> usize {
        match self {
            Node::Branch {
                name: _,
                weight,
                children: _,
            } => *weight,
            _ => self.weight(),
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Node::RawBranch {
                name: _,
                weight: _,
                children: _,
            } => false,
            Node::Branch {
                name: _,
                weight: _,
                children: _,
            } => false,
            Node::Leaf { name: _, weight: _ } => true,
        }
    }

    fn is_balanced(&self) -> bool {
        match self {
            Node::RawBranch {
                name: _,
                weight: _,
                children: _,
            } => panic!("RawBranch can't be balanced!"),
            Node::Branch {
                name: _,
                weight: _,
                children,
            } => {
                children
                    .iter()
                    .map(|child| child.weight())
                    .collect::<HashSet<usize>>()
                    .len()
                    == 1
            }
            Node::Leaf { name: _, weight: _ } => true,
        }
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut raw_nodes: HashMap<String, Node> = value
            .lines()
            .map(|line| Self::from_single(line))
            .map(|node| (node.name().to_owned(), node))
            .collect();

        let root = find_root(&raw_nodes);
        build_tree(&mut raw_nodes, &root)
    }
}

fn find_root(raw_nodes: &HashMap<String, Node>) -> String {
    let mut names: HashSet<String> = raw_nodes.keys().map(|node| node.to_owned()).collect();

    for raw_node in raw_nodes.values() {
        match raw_node {
            Node::RawBranch {
                name: _,
                weight: _,
                children,
            } => children.iter().for_each(|child_name| {
                names.remove(child_name);
            }),
            Node::Leaf { name, weight: _ } => {
                names.remove(name);
            }
            _ => (),
        };
    }

    assert_eq!(names.len(), 1);
    names.into_iter().next().unwrap()
}

fn build_tree(raw_nodes: &mut HashMap<String, Node>, name: &str) -> Node {
    let raw_node = raw_nodes.remove(name).unwrap();

    match &raw_node {
        Node::Leaf { name: _, weight: _ } => raw_node,
        Node::RawBranch {
            name,
            weight,
            children,
        } => {
            let children: Vec<Node> = children
                .iter()
                .map(|child| build_tree(raw_nodes, child.as_str()))
                .collect();
            Node::Branch {
                name: name.clone(),
                weight: *weight,
                children,
            }
        }
        _ => panic!("Illegal state!"),
    }
}

fn select_unbalanced_child(node: &Node) -> (&Node, usize) {
    let children = match node {
        Node::Branch {
            name: _,
            weight: _,
            children,
        } => children,
        _ => panic!("Only Branches allowed!"),
    };

    let histogram =
        children
            .iter()
            .map(|child| child.weight())
            .fold(HashMap::new(), |mut acc, w| {
                if let Some(count) = acc.get(&w) {
                    acc.insert(w, count + 1);
                } else {
                    acc.insert(w, 1);
                }
                acc
            });

    let mut balanced_weight: Vec<(usize, usize)> =
        histogram.iter().map(|(w, count)| (*w, *count)).collect();
    balanced_weight.sort_by(|a, b| b.1.cmp(&a.1));
    let balanced_weight = balanced_weight.first().unwrap().0;

    let (unbalanced_weight, _) = histogram.iter().fold((0, usize::MAX), |acc, (w, count)| {
        if *count < acc.1 {
            (*w, *count)
        } else {
            acc
        }
    });

    let unbalanced_node = children
        .iter()
        .filter(|n| n.weight() == unbalanced_weight)
        .next()
        .unwrap();

    let unbalanced_weight: isize = unbalanced_weight.try_into().unwrap();
    let balanced_weight: isize = balanced_weight.try_into().unwrap();
    let weight: isize = unbalanced_node.self_weight().try_into().unwrap();

    let diff = unbalanced_weight - balanced_weight;
    let weight: usize = (weight - diff).try_into().unwrap();

    (unbalanced_node, weight)
}

fn find_unbalanced_node(node: &Node) -> Option<(&Node, usize)> {
    match node {
        Node::Branch {
            name: _,
            weight: _,
            children,
        } => {
            for child in children {
                if let Some(unbalanced) = find_unbalanced_node(child) {
                    return Some(unbalanced);
                }
            }
            if !node.is_balanced() {
                return Some(select_unbalanced_child(node));
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_unbalanced_node, Node};

    #[test]
    fn test_examples() {
        let programs = "\
            pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)\
        ";

        let root = Node::from(programs);
        assert_eq!(root.name(), "tknk");

        let unbalanced = find_unbalanced_node(&root);
        assert!(unbalanced.is_some());
        let unbalanced = unbalanced.unwrap();
        assert_eq!(unbalanced.0.name(), "ugml");
        assert_eq!(unbalanced.1, 60);
    }

    #[test]
    fn test_input() {
        let programs = std::fs::read_to_string("input/programs.txt").unwrap();

        let root = Node::from(programs.as_str());
        assert_eq!(root.name(), "ykpsek");

        let unbalanced = find_unbalanced_node(&root);
        assert!(unbalanced.is_some());
        let unbalanced = unbalanced.unwrap();
        assert_eq!(unbalanced.0.name(), "cumah");
        assert_eq!(unbalanced.1, 1060);
    }
}
