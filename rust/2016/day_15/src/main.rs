#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2016 - day 15");
}

lazy_static! {
    static ref RE_DISC: Regex =
        Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=(\d+), it is at position (\d+).$")
            .unwrap();
}

#[derive(Debug)]
struct Disc {
    number: usize,
    positions: usize,
    pos_at_t0: usize,
}

impl From<&str> for Disc {
    fn from(value: &str) -> Self {
        let caps = RE_DISC
            .captures(value)
            .expect(&format!("Input did not match RE_DISC: {}", value));

        let number = caps[1]
            .parse::<usize>()
            .expect("Disc number should parse to usize!");
        let positions = caps[2]
            .parse::<usize>()
            .expect("Positions should parse to usize!");

        let initial_t = caps[3]
            .parse::<usize>()
            .expect("Time should parse to usize!");
        let initial_pos = caps[4]
            .parse::<usize>()
            .expect("Initial position should parse to usize!");

        let initial_t = initial_t % positions;
        let pos_at_t0 = ((initial_pos + positions) - initial_t) % positions;

        Self {
            number,
            positions,
            pos_at_t0,
        }
    }
}

impl Disc {
    fn open_when_button_pressed_at(&self, t: usize) -> bool {
        ((self.pos_at_t0 + self.number + t) % self.positions) == 0
    }
}

#[derive(Debug)]
struct Machine {
    discs: Vec<Disc>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let discs = value.lines().map(|line| Disc::from(line)).collect();

        Self { discs }
    }
}

impl Machine {
    fn first_opportunity_for_success(&self) -> usize {
        let mut i = 0;
        loop {
            let success = self
                .discs
                .iter()
                .map(|disc| disc.open_when_button_pressed_at(i))
                .fold(true, |acc, b| acc && b);
            if success {
                return i;
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Machine;

    #[test]
    fn test_example() {
        let machine = "\
            Disc #1 has 5 positions; at time=0, it is at position 4.\n\
            Disc #2 has 2 positions; at time=0, it is at position 1.\
        ";
        let machine = Machine::from(machine);
        assert_eq!(machine.first_opportunity_for_success(), 5);
    }

    #[test]
    fn test_input() {
        let machine_file = std::fs::read_to_string("input/discs.txt").unwrap();
        let machine = Machine::from(&machine_file[0..]);
        assert_eq!(machine.first_opportunity_for_success(), 16824);

        let machine_part2 = format!(
            "{}\n{}",
            &machine_file, "Disc #7 has 11 positions; at time=0, it is at position 0."
        );
        let machine = Machine::from(&machine_part2[0..]);
        assert_eq!(machine.first_opportunity_for_success(), 3543984);
    }
}
