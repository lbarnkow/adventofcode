#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2019 - day 03");
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn dist(&self, other: &Pos) -> usize {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);
        x_dist + y_dist
    }
}

impl std::ops::Mul<usize> for Pos {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::Output {
            x: self.x * isize::try_from(rhs).unwrap(),
            y: self.y * isize::try_from(rhs).unwrap(),
        }
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, rhs: Pos) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Dir: illegal character: {value}"),
        }
    }
}

impl From<Dir> for Pos {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Pos::new(0, -1),
            Dir::Down => Pos::new(0, 1),
            Dir::Left => Pos::new(-1, 0),
            Dir::Right => Pos::new(1, 0),
        }
    }
}

impl From<Dir> for char {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up | Dir::Down => '|',
            Dir::Left | Dir::Right => '-',
        }
    }
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    len: usize,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        let dir = value[0..1].chars().next().unwrap().into();
        let len = value[1..].parse().unwrap();

        Self { dir, len }
    }
}

impl From<&Step> for Pos {
    fn from(value: &Step) -> Self {
        Pos::from(value.dir) * value.len
    }
}

impl From<Step> for Pos {
    fn from(value: Step) -> Self {
        Pos::from(&value)
    }
}

#[derive(Debug)]
struct Wire {
    steps: Vec<Step>,
}

impl From<&str> for Wire {
    fn from(value: &str) -> Self {
        let steps = value.split(',').map(|step| step.into()).collect();
        Self { steps }
    }
}

impl Wire {
    fn len(&self) -> usize {
        self.steps.iter().map(|step| step.len).sum::<usize>() + 1
    }

    fn segments(&self) -> HashMap<Pos, usize> {
        let mut steps = 0;
        let mut segments = HashMap::with_capacity(self.len());

        let mut pos = Pos::default();
        segments.insert(pos, steps);

        for step in &self.steps {
            let offset = Pos::from(step.dir);
            for _ in 0..step.len {
                steps += 1;
                pos += offset;
                if let Some(old_steps) = segments.insert(pos, steps) {
                    segments.insert(pos, old_steps);
                }
            }
        }

        segments
    }
}

struct WirePanel {
    wires: [Wire; 2],
}

impl From<&str> for WirePanel {
    fn from(value: &str) -> Self {
        let mut iter = value.lines();
        let wire1 = iter.next().unwrap().into();
        let wire2 = iter.next().unwrap().into();
        assert_eq!(iter.next(), None);

        Self::new([wire1, wire2])
    }
}

impl WirePanel {
    fn new(wires: [Wire; 2]) -> Self {
        Self { wires }
    }

    fn intersections(&self) -> HashMap<Pos, (usize, usize)> {
        let wire_1_segments = self.wires[0].segments();
        let mut intersections = HashMap::new();

        for (pos, wire_2_steps) in self.wires[1].segments() {
            if let Some(wire_1_steps) = wire_1_segments.get(&pos) {
                intersections.insert(pos, (*wire_1_steps, wire_2_steps));
            }
        }

        intersections.remove(&Pos::default());
        intersections.into_iter().collect()
    }

    fn dist_to_closest_intersection(&self) -> usize {
        let origin = Pos::default();
        self.intersections()
            .into_keys()
            .map(|p| p.dist(&origin))
            .min()
            .unwrap()
    }

    fn combined_fewest_steps_to_intersection(&self) -> usize {
        self.intersections()
            .into_values()
            .map(|(wire_1_steps, wire_2_steps)| wire_1_steps + wire_2_steps)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Pos, WirePanel};

    #[test]
    fn test_examples() {
        let wires = "\
            R8,U5,L5,D3\n\
            U7,R6,D4,L4\
        ";
        let panel = WirePanel::from(wires);

        let intersections = panel.intersections();
        assert_eq!(intersections.len(), 2);
        assert!(intersections.contains_key(&Pos::new(3, -3)));
        assert!(intersections.contains_key(&Pos::new(6, -5)));

        assert_eq!(panel.dist_to_closest_intersection(), 6);
        assert_eq!(panel.combined_fewest_steps_to_intersection(), 30);

        let wires = "\
            R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
            U62,R66,U55,R34,D71,R55,D58,R83\
        ";
        let panel = WirePanel::from(wires);
        assert_eq!(panel.dist_to_closest_intersection(), 159);
        assert_eq!(panel.combined_fewest_steps_to_intersection(), 610);

        let wires = "\
            R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7\
        ";
        let panel = WirePanel::from(wires);
        assert_eq!(panel.dist_to_closest_intersection(), 135);
        assert_eq!(panel.combined_fewest_steps_to_intersection(), 410);
    }

    #[test]
    fn test_input() {
        let wires = std::fs::read_to_string("input/wires.txt").unwrap();
        let panel = WirePanel::from(wires.as_str());

        assert_eq!(panel.dist_to_closest_intersection(), 5357);
        assert_eq!(panel.combined_fewest_steps_to_intersection(), 101956);
    }
}
