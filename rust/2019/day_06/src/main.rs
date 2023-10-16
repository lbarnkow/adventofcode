#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Display,
    rc::{Rc, Weak},
};

fn main() {
    println!("Advent of Code 2019 - day 06");
}

#[derive(Debug)]
struct MapNode {
    parent: Option<Weak<RefCell<MapNode>>>,
    name: String,
    children: Vec<Rc<RefCell<MapNode>>>,
}

impl MapNode {
    fn new(
        name: &str,
        orbits: &mut HashMap<&str, Vec<&str>>,
        parent: Option<Weak<RefCell<MapNode>>>,
    ) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            parent,
            name: name.to_string(),
            children: Vec::with_capacity(0),
        }));

        let children = orbits
            .remove(name)
            .unwrap_or(vec![])
            .iter()
            .map(|child_name| Self::new(child_name, orbits, Some(Rc::downgrade(&this))))
            .collect();

        this.borrow_mut().children = children;
        this
    }

    fn print(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) {
        writeln!(f, "{:indent$}- {}", "", &self.name).unwrap();
        for child in &self.children {
            child.borrow().print(f, indent + 2);
        }
    }

    fn total_orbits_go(&self, depth: usize) -> usize {
        self.children
            .iter()
            .map(|child| child.borrow().total_orbits_go(depth + 1))
            .sum::<usize>()
            + depth
    }

    fn total_orbits(&self) -> usize {
        self.total_orbits_go(0)
    }

    fn from(value: &str) -> Rc<RefCell<Self>> {
        let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();

        for line in value.lines() {
            let mut iter = line.split(')');
            let k = iter.next().unwrap();
            let v = iter.next().unwrap();
            if let Some(children) = orbits.get_mut(k) {
                children.push(v);
            } else {
                orbits.insert(k, vec![v]);
            }
        }

        let root = MapNode::new("COM", &mut orbits, None);
        assert!(orbits.is_empty());
        root
    }

    fn find_node(root: Rc<RefCell<MapNode>>, name: &str) -> Rc<RefCell<MapNode>> {
        let mut visited = Vec::new();
        visited.push(root.borrow().name.clone());
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(current) = queue.pop_front() {
            if current.borrow().name == name {
                return current;
            }
            let current = current.borrow();

            if let Some(parent) = &current.parent {
                let parent = parent.upgrade().unwrap();
                if !visited.contains(&parent.borrow().name) {
                    visited.push(parent.borrow().name.clone());
                    queue.push_back(parent);
                }
            }

            for child in &current.children {
                if !visited.contains(&child.borrow().name) {
                    visited.push(child.borrow().name.clone());
                    queue.push_back(child.clone());
                }
            }
        }

        panic!("Node not found!");
    }

    fn dist_between(root: Rc<RefCell<MapNode>>, a: &str, b: &str) -> usize {
        let a = Self::find_node(root.clone(), a)
            .borrow()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        let b = Self::find_node(root.clone(), b)
            .borrow()
            .parent
            .clone()
            .unwrap()
            .upgrade()
            .unwrap();
        let b = b.borrow().name.clone();

        let mut visited = Vec::new();
        visited.push(a.borrow().name.clone());
        let mut queue = VecDeque::new();
        queue.push_back((a, 0));

        while let Some((current, steps)) = queue.pop_front() {
            let current = current.borrow();
            if current.name == b {
                return steps;
            }

            if let Some(parent) = &current.parent {
                let parent = parent.upgrade().unwrap();
                if !visited.contains(&parent.borrow().name) {
                    visited.push(parent.borrow().name.clone());
                    queue.push_back((parent, steps + 1));
                }
            }

            for child in &current.children {
                if !visited.contains(&child.borrow().name) {
                    visited.push(child.borrow().name.clone());
                    queue.push_back((child.clone(), steps + 1));
                }
            }
        }

        panic!("No route found!");
    }
}

impl Display for MapNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f, 0);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::MapNode;

    #[test]
    fn test_examples() {
        let orbits = "\
            COM)B\n\
            B)C\n\
            C)D\n\
            D)E\n\
            E)F\n\
            B)G\n\
            G)H\n\
            D)I\n\
            E)J\n\
            J)K\n\
            K)L\
        ";
        let root = MapNode::from(orbits);
        assert_eq!(root.borrow().total_orbits(), 42);

        let orbits = "\
            COM)B\n\
            B)C\n\
            C)D\n\
            D)E\n\
            E)F\n\
            B)G\n\
            G)H\n\
            D)I\n\
            E)J\n\
            J)K\n\
            K)L\n\
            K)YOU\n\
            I)SAN\
        ";
        let root = MapNode::from(orbits);
        let san = MapNode::find_node(root.clone(), "SAN");
        assert_eq!(san.borrow().name, "SAN");
        assert_eq!(
            san.borrow()
                .parent
                .clone()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .name,
            "I"
        );
        assert_eq!(MapNode::dist_between(root.clone(), "YOU", "SAN"), 4);
    }

    #[test]
    fn test_input() {
        let orbits = std::fs::read_to_string("input/orbits.txt").unwrap();
        let root = MapNode::from(orbits.as_str());
        assert_eq!(root.borrow().total_orbits(), 234446);
        assert_eq!(MapNode::dist_between(root.clone(), "YOU", "SAN"), 385);
    }
}
