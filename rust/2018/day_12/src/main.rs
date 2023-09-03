#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2018 - day 12");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pot {
    Plant,
    Empty,
}

impl From<char> for Pot {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Plant,
            '.' => Self::Empty,
            _ => panic!("Illegal pot character: {value}!"),
        }
    }
}

impl Display for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pot::Plant => '#',
            Pot::Empty => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Pots {
    left: Vec<Pot>,
    right: Vec<Pot>,
}

impl From<&str> for Pots {
    fn from(value: &str) -> Self {
        assert!(value.starts_with("initial state: "));
        let data = (&value[15..]).chars().map(|c| Pot::from(c)).collect();
        let mut pots = Self {
            left: Vec::new(),
            right: data,
        };
        pots.clean_ends();
        pots
    }
}

impl Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pot in self.left.iter().rev() {
            write!(f, "{}", pot).unwrap();
        }
        for pot in &self.right {
            write!(f, "{}", pot).unwrap();
        }
        Ok(())
    }
}

impl Pots {
    fn get(&self, i: isize) -> Pot {
        if i < 0 {
            let idx = i.abs() as usize - 1;
            if idx < self.left.len() {
                return self.left[idx];
            }
        } else {
            let idx = i as usize;
            if idx < self.right.len() {
                return self.right[idx];
            }
        }
        Pot::Empty
    }

    fn clean_ends(&mut self) {
        while !self.left.is_empty() && *self.left.last().unwrap() == Pot::Empty {
            self.left.pop();
        }
        while !self.right.is_empty() && *self.right.last().unwrap() == Pot::Empty {
            self.right.pop();
        }
    }

    fn apply(&mut self, rules: &[Rule]) {
        let mut left = Vec::with_capacity(self.left.len());
        let mut right = Vec::with_capacity(self.right.len());

        for idx in 0..self.left.len() + 2 {
            let idx = -(idx as isize + 1);
            left.push(Rule::apply(rules, self, idx));
        }

        for idx in 0..self.right.len() + 2 {
            right.push(Rule::apply(rules, self, idx as isize));
        }

        self.left = left;
        self.right = right;

        self.clean_ends();
    }

    fn apply_n(&mut self, rules: &[Rule], n: u64) -> i64 {
        let mut prev_count = self.count();
        let mut prev_score = self.score();
        let mut streak = 1;

        let mut remaining = 0;

        for t in 1..=n {
            self.apply(rules);
            let count = self.count();
            if count == prev_count {
                streak += 1;
                if streak == 10 {
                    remaining = n - t;
                    break;
                }
                prev_score = self.score();
            } else {
                prev_count = count;
                streak = 0;
            }
        }

        let score_diff = self.score() - prev_score;
        self.score() + score_diff * (remaining as i64)
    }

    fn count(&self) -> usize {
        self.left.iter().filter(|p| **p == Pot::Plant).count()
            + self.right.iter().filter(|p| **p == Pot::Plant).count()
    }

    fn score(&self) -> i64 {
        let left_score: i64 = self
            .left
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pot::Plant)
            .map(|(idx, _)| -((idx + 1) as i64))
            .sum();

        let right_score: i64 = self
            .right
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Pot::Plant)
            .map(|(idx, _)| idx as i64)
            .sum();

        left_score + right_score
    }
}

#[derive(Debug)]
struct Rule {
    lhs: [Pot; 5],
    rhs: Pot,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut split = value.split("=>");

        let lhs = split.next().unwrap().trim();
        let rhs = split.next().unwrap().trim();

        let lhs: Vec<Pot> = lhs.chars().map(|c| c.into()).collect();

        Self {
            lhs: lhs.try_into().unwrap(),
            rhs: rhs.chars().next().unwrap().into(),
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pot in &self.lhs {
            write!(f, "{}", pot).unwrap();
        }
        write!(f, " => {}", self.rhs)
    }
}

impl Rule {
    fn apply(rules: &[Rule], pots: &Pots, idx: isize) -> Pot {
        let window = [
            pots.get(idx - 2),
            pots.get(idx - 1),
            pots.get(idx),
            pots.get(idx + 1),
            pots.get(idx + 2),
        ];
        for rule in rules {
            if rule.lhs == window {
                return rule.rhs;
            }
        }
        // If no rule matches -> produce an empty Pot
        Pot::Empty
    }
}

fn parse_input(s: &str) -> (Pots, Vec<Rule>) {
    let mut s = s.lines();

    let pots = Pots::from(s.next().unwrap());
    s.next().unwrap();
    let rules = s
        .map(|line| line.into())
        .filter(|r: &Rule| r.rhs != Pot::Empty)
        .collect();

    (pots, rules)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn test_examples() {
        let input = "\
            initial state: #..#.#..##......###...###\n\
            \n\
            ...## => #\n\
            ..#.. => #\n\
            .#... => #\n\
            .#.#. => #\n\
            .#.## => #\n\
            .##.. => #\n\
            .#### => #\n\
            #.#.# => #\n\
            #.### => #\n\
            ##.#. => #\n\
            ##.## => #\n\
            ###.. => #\n\
            ###.# => #\n\
            ####. => #\
        ";

        let (mut pots, rules) = parse_input(input);

        assert_eq!(pots.to_string(), "#..#.#..##......###...###");

        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#...#....#.....#..#..#..#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "##..##...##....#..#..#..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#.#...#..#.#....#..#..#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#.#..#...#.#...#..#..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), ".#...##...#.#..#..#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), ".##.#.#....#...#..##..##..##");

        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#..###.#...##..#...#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#....##.#.#.#..##..##..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "##..#..#####....#...#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#.#..#...#.##....##..##..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#...##...#.#...#.#...#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "##.#.#....#.#...#.#..##..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#..###.#....#.#...#....#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#....##.#....#.#..##...##..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "##..#..#.#....#....#..#.#...#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#.#..#...#.#...##...#...#.#..##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#...##...#.#.#.#...##...#....#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "##.#.#....#####.#.#.#...##...##..##");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#..###.#..#.#.#######.#.#.#..#.#...#");
        pots.apply(&rules);
        assert_eq!(pots.to_string(), "#....##....#####...#######....#.#..##");

        assert_eq!(pots.score(), 325);

        // part 2
        let (mut pots, rules) = parse_input(input);
        assert_eq!(pots.to_string(), "#..#.#..##......###...###");
        pots.apply_n(&rules, 20);
        assert_eq!(pots.to_string(), "#....##....#####...#######....#.#..##");
        assert_eq!(pots.score(), 325);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/plants.txt").unwrap();
        let (mut pots, rules) = parse_input(&input);
        assert_eq!(pots.to_string(), "#...#..###.#.###.####.####.#..#.##..#..##..#.....#.#.#.##.#...###.#..##..#.##..###..#..##.#..##");

        for _ in 0..20 {
            pots.apply(&rules);
        }
        assert_eq!(pots.score(), 1787);

        // part 2
        let (mut pots, rules) = parse_input(&input);
        assert_eq!(pots.to_string(), "#...#..###.#.###.####.####.#..#.##..#..##..#.....#.#.#.##.#...###.#..##..#.##..###..#..##.#..##");
        let score = pots.apply_n(&rules, 50_000_000_000);
        assert_eq!(score, 1_100_000_000_475);
    }
}
