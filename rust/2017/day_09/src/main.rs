#![allow(dead_code)]

use std::{ops::AddAssign, str::Chars};

fn main() {
    println!("Advent of Code 2017 - day 09");
}

#[derive(Default)]
struct Stats {
    groups: usize,
    group_score: usize,
    garbage_count: usize,
}

impl Stats {
    fn from_group(score: usize) -> Self {
        Self {
            groups: 1,
            group_score: score,
            garbage_count: 0,
        }
    }

    fn from_garbage(garbage_count: usize) -> Self {
        Self {
            groups: 0,
            group_score: 0,
            garbage_count,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.groups += rhs.groups;
        self.group_score += rhs.group_score;
        self.garbage_count += rhs.garbage_count;
    }
}

struct PeekableChars<'a> {
    iter: Chars<'a>,
    next: Option<char>,
}

impl<'a> PeekableChars<'a> {
    fn new(mut iter: Chars<'a>) -> Self {
        let next = iter.next();
        Self { iter, next }
    }

    fn peek(&self) -> Option<char> {
        self.next.clone()
    }

    fn next(&mut self) -> Option<char> {
        let tmp = self.next;
        self.next = self.iter.next();
        tmp
    }
}

fn parse_garbage(chars: &mut PeekableChars<'_>) -> Stats {
    let mut garbage_count = 0;

    while let Some(c) = chars.next() {
        match c {
            '>' => return Stats::from_garbage(garbage_count),
            '!' => {
                chars.next().expect("Should ignore next character!");
                ()
            }
            _ => garbage_count += 1,
        };
    }

    panic!("Garbage not terminated properly!")
}

fn parse_group(chars: &mut PeekableChars<'_>, depth: usize) -> Stats {
    let mut stats = Stats::from_group(depth);

    while let Some(c) = chars.next() {
        match c {
            '{' | '<' => {
                if c == '{' {
                    stats += parse_group(chars, depth + 1);
                } else {
                    stats += parse_garbage(chars);
                }
                if let Some(',') = chars.peek() {
                    chars.next(); // consume comma
                    if let Some('{') = chars.peek() {
                        // that's good – will open new group after a comma
                    } else if let Some('<') = chars.peek() {
                        // that's good – will open new group after a comma
                    } else {
                        panic!("Illegal character after group item closure and comma: '{c}'!");
                    }
                } else if let Some('}') = chars.peek() {
                    // that's good – closes this group
                } else {
                    panic!("Illegal character after group: '{c}'!");
                }
            }
            '}' => break,
            _ => panic!("Illegal character in group: '{c}'!"),
        }
    }

    stats
}

fn parse(s: &str) -> Stats {
    let mut chars = PeekableChars::new(s.chars());
    let mut stats = Stats::default();

    while let Some(c) = chars.next() {
        match c {
            '{' => stats += parse_group(&mut chars, 1),
            '<' => stats += parse_garbage(&mut chars),
            _ => panic!("Illegal character in main loop: '{c}'!"),
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_examples() {
        assert_eq!(parse("{}").groups, 1);
        assert_eq!(parse("{{{}}}").groups, 3);
        assert_eq!(parse("{{},{}}").groups, 3);
        assert_eq!(parse("{{{},{},{{}}}}").groups, 6);
        assert_eq!(parse("{<a>,<a>,<a>,<a>}").groups, 1);
        assert_eq!(parse("{{<a>},{<a>},{<a>},{<a>}}").groups, 5);
        assert_eq!(parse("{{<!>},{<!>},{<!>},{<a>}}").groups, 2);

        assert_eq!(parse("{}").group_score, 1);
        assert_eq!(parse("{{{}}}").group_score, 6);
        assert_eq!(parse("{{},{}}").group_score, 5);
        assert_eq!(parse("{{{},{},{{}}}}").group_score, 16);
        assert_eq!(parse("{<a>,<a>,<a>,<a>}").group_score, 1);
        assert_eq!(parse("{{<ab>},{<ab>},{<ab>},{<ab>}}").group_score, 9);
        assert_eq!(parse("{{<!!>},{<!!>},{<!!>},{<!!>}}").group_score, 9);
        assert_eq!(parse("{{<a!>},{<a!>},{<a!>},{<ab>}}").group_score, 3);

        assert_eq!(parse("<>").garbage_count, 0);
        assert_eq!(parse("<random characters>").garbage_count, 17);
        assert_eq!(parse("<<<<>").garbage_count, 3);
        assert_eq!(parse("<{!>}>").garbage_count, 2);
        assert_eq!(parse("<!!>").garbage_count, 0);
        assert_eq!(parse("<!!!>>").garbage_count, 0);
        assert_eq!(parse("<{o\"i!a,<{i<a>").garbage_count, 10);
    }

    #[test]
    fn test_input() {
        let stream = std::fs::read_to_string("input/stream.txt").unwrap();
        let stats = parse(&stream);
        assert_eq!(stats.group_score, 12505);
        assert_eq!(stats.garbage_count, 6671);
    }
}
