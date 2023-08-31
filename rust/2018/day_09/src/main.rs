#![allow(dead_code)]

use std::collections::VecDeque;

use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 09");
}

struct Params {
    players: u64,
    max_marble: u64,
}

impl From<&str> for Params {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();
        let caps = re.captures(value).unwrap();

        Self {
            players: caps[1].parse().unwrap(),
            max_marble: caps[2].parse().unwrap(),
        }
    }
}

fn compute_highscore(params: &Params) -> u64 {
    let mut circle = VecDeque::with_capacity(params.max_marble as usize);
    circle.push_back(0);

    let mut scores = vec![0; params.players as usize];
    let mut active_player = 0;

    for marble in 1..=params.max_marble {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            scores[active_player] += marble + circle.pop_front().unwrap();
        } else {
            if circle.len() > 1 {
                circle.rotate_left(2)
            }
            circle.push_front(marble);
        }
        active_player = (active_player + 1) % (params.players as usize);
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{compute_highscore, Params};

    #[test]
    fn test_examples() {
        let params_list = "\
            9 players; last marble is worth 25 points\n\
            10 players; last marble is worth 1618 points\n\
            13 players; last marble is worth 7999 points\n\
            17 players; last marble is worth 1104 points\n\
            21 players; last marble is worth 6111 points\n\
            30 players; last marble is worth 5807 points\
        ";
        let mut highscores = [32, 8317, 146373, 2764, 54718, 37305].into_iter();

        for params in params_list.lines().map(|line| Params::from(line)) {
            let highscore = compute_highscore(&params);
            let expected_highscore = highscores.next().unwrap();
            println!(
                "{} - {} - {} - {}",
                params.players, params.max_marble, highscore, expected_highscore
            );
            assert_eq!(highscore, expected_highscore);
        }
    }

    #[test]
    fn test_input() {
        let params = std::fs::read_to_string("input/params.txt").unwrap();
        let mut params = Params::from(params.as_str());

        let highscore = compute_highscore(&params);
        assert_eq!(highscore, 398048);

        params.max_marble *= 100;
        let highscore = compute_highscore(&params);
        assert_eq!(highscore, 3180373421);
    }
}
