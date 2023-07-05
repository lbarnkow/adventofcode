#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 6");
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"(turn on|toggle|turn off)\s(\d+),(\d+)\sthrough\s(\d+).(\d+)").unwrap();
}

enum Command {
    TurnOn,
    Toggle,
    TurnOff,
}

impl Command {
    fn parse(s: &str) -> Self {
        match s {
            "turn on" => Command::TurnOn,
            "toggle" => Command::Toggle,
            "turn off" => Command::TurnOff,
            _ => panic!("Unsupported command!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn parse(x: &str, y: &str) -> Self {
        Self {
            x: usize::from_str_radix(x, 10).unwrap(),
            y: usize::from_str_radix(y, 10).unwrap(),
        }
    }
}

fn parse_line(line: &str) -> (Command, Pos, Pos) {
    let cap = REGEX.captures(line).unwrap();

    (
        Command::parse(&cap[1]),
        Pos::parse(&cap[2], &cap[3]),
        Pos::parse(&cap[4], &cap[5]),
    )
}

fn count_lights(lines: &[&str]) -> usize {
    let mut lights = [[false; 1000]; 1000];

    for line in lines {
        let (cmd, from, to) = parse_line(line);

        for x in from.x..=to.x {
            for y in from.y..=to.y {
                match cmd {
                    Command::TurnOn => lights[x][y] = true,
                    Command::Toggle => lights[x][y] = !lights[x][y],
                    Command::TurnOff => lights[x][y] = false,
                }
            }
        }
    }

    lights
        .iter()
        .map(|col| col.iter().filter(|cell| **cell).count())
        .sum()
}

fn calculate_brightness(lines: &[&str]) -> usize {
    let mut lights = vec![vec![0; 1000]; 1000];

    for line in lines {
        let (cmd, from, to) = parse_line(line);

        for x in from.x..=to.x {
            for y in from.y..=to.y {
                match cmd {
                    Command::TurnOn => lights[x][y] += 1,
                    Command::Toggle => lights[x][y] += 2,
                    Command::TurnOff => {
                        if lights[x][y] > 0 {
                            lights[x][y] -= 1;
                        } else {
                            lights[x][y] = 0;
                        }
                    }
                }
            }
        }
    }

    lights.iter().map(|col| col.iter().sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_brightness, count_lights};

    #[test]
    fn test_examples() {
        assert_eq!(count_lights(&["turn on 0,0 through 999,999"]), 1000000);

        assert_eq!(
            count_lights(&["turn on 0,0 through 999,999", "toggle 0,0 through 999,0"]),
            999000
        );

        assert_eq!(
            count_lights(&[
                "turn on 0,0 through 999,999",
                "toggle 0,0 through 999,0",
                "turn off 499,499 through 500,500"
            ]),
            998996
        );
    }

    #[test]
    fn test_input() {
        let commands = std::fs::read_to_string("input/commands.txt").unwrap();
        assert_eq!(
            count_lights(commands.lines().collect::<Vec<&str>>().as_slice()),
            569999
        );
        assert_eq!(
            calculate_brightness(commands.lines().collect::<Vec<&str>>().as_slice()),
            17836115
        );
    }
}
