#![allow(dead_code)]

use std::{
    collections::VecDeque,
    sync::mpsc::{self, Sender},
    thread,
};

fn main() {
    println!("Advent of Code 2018 - day 05");
}

#[inline]
fn do_polymers_react(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a != b
}

fn collapse(suit: &str) -> String {
    let mut buffer = VecDeque::with_capacity(suit.len());

    for c in suit.chars() {
        if let Some(prev) = buffer.pop_back() {
            if !do_polymers_react(prev, c) {
                buffer.push_back(prev);
                buffer.push_back(c);
            }
        } else {
            buffer.push_back(c);
        }
    }

    buffer.iter().collect()
}

fn fix_and_collapse_go(
    suit: &str,
    ignored: char,
    buffer: &mut VecDeque<char>,
    tx: &mut Sender<(char, String)>,
) {
    let ignored_u = ignored.to_ascii_uppercase();
    buffer.clear();
    for c in suit.chars() {
        if c == ignored || c == ignored_u {
            continue;
        }
        if let Some(prev) = buffer.pop_back() {
            if !do_polymers_react(prev, c) {
                buffer.push_back(prev);
                buffer.push_back(c);
            }
        } else {
            buffer.push_back(c);
        }
    }

    let collapsed = buffer.iter().collect();
    tx.send((ignored, collapsed)).unwrap();
}

fn fix_and_collapse(suit: &str) -> (char, String) {
    let ignored_count = ('a'..='z').count();
    let t_count = thread::available_parallelism().unwrap().get();

    let ignored_chars = ('a'..='z').collect::<Vec<char>>();
    let chunks = ignored_chars.chunks(ignored_count / t_count);

    let (tx, rx) = mpsc::channel();

    for chunk in chunks {
        let chunk = String::from_iter(chunk);
        let mut tx = tx.clone();
        let suit = suit.to_owned();
        thread::spawn(move || {
            let mut buffer = VecDeque::with_capacity(suit.len());
            for c in chunk.chars() {
                fix_and_collapse_go(&suit, c, &mut buffer, &mut tx);
            }
        });
    }
    drop(tx);

    rx.into_iter()
        .min_by(|(_, s1), (_, s2)| s1.len().cmp(&s2.len()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{collapse, fix_and_collapse};

    #[test]
    fn test_examples() {
        let suit = "aA";
        let collapsed = collapse(suit);
        assert_eq!(collapsed.len(), 0);
        assert_eq!(collapsed, "");

        let suit = "abBA";
        let collapsed = collapse(suit);
        assert_eq!(collapsed.len(), 0);
        assert_eq!(collapsed, "");

        let suit = "abAB";
        let collapsed = collapse(suit);
        assert_eq!(collapsed.len(), 4);
        assert_eq!(collapsed, "abAB");

        let suit = "aabAAB";
        let collapsed = collapse(suit);
        assert_eq!(collapsed.len(), 6);
        assert_eq!(collapsed, "aabAAB");

        let suit = "dabAcCaCBAcCcaDA";
        let collapsed = collapse(suit);
        assert_eq!(collapsed.len(), 10);
        assert_eq!(collapsed, "dabCBAcaDA");

        let (ignored, collapsed) = fix_and_collapse(suit);
        assert_eq!(ignored, 'c');
        assert_eq!(collapsed, "daDA");
    }

    #[test]
    fn test_input() {
        let suit = std::fs::read_to_string("input/suit.txt").unwrap();

        let collapsed = collapse(&suit);
        assert_eq!(collapsed.len(), 11540);

        let (ignored, collapsed) = fix_and_collapse(&suit);
        assert_eq!(ignored, 'b');
        assert_eq!(collapsed.len(), 6918);
    }
}
