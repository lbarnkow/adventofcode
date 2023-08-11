#![allow(dead_code)]

use std::{collections::VecDeque, fmt::Display};

fn main() {
    println!("Advent of Code 2017 - day 13");
}

#[derive(Debug)]
enum Dir {
    Forward,
    Backward,
}

impl Dir {
    fn flip(&self) -> Self {
        match self {
            Dir::Forward => Dir::Backward,
            Dir::Backward => Dir::Forward,
        }
    }
}

#[derive(Debug)]
struct Scanner {
    depth: usize,
    range: usize,
    pos: usize,
    dir: Dir,
}

impl Scanner {
    fn reset(&mut self) {
        self.pos = 0;
        self.dir = Dir::Forward;
    }

    fn step(&mut self) {
        if self.range == 1 {
            return;
        }

        match self.dir {
            Dir::Forward => self.pos += 1,
            Dir::Backward => self.pos -= 1,
        }

        if self.pos == 0 || self.pos == self.range - 1 {
            self.dir = self.dir.flip();
        }
    }

    fn severity(&self) -> usize {
        self.depth * self.range
    }
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        let mut split = value.split(": ");
        let depth = split.next().unwrap().parse::<usize>().unwrap();
        let range = split.next().unwrap().parse::<usize>().unwrap();
        Self {
            depth,
            range,
            pos: 0,
            dir: Dir::Forward,
        }
    }
}

#[derive(Debug)]
struct Packet {
    start_t: usize,
    online: bool,
    severity: usize,
    pos: usize,
    caught: bool,
}

impl Packet {
    fn new(start_t: usize) -> Self {
        Self {
            start_t,
            online: false,
            severity: 0,
            pos: 0,
            caught: false,
        }
    }

    fn step(&mut self) {
        self.pos += 1;
    }
}

#[derive(Debug)]
struct Firewall {
    t: usize,
    length: usize,
    scanners: Vec<Scanner>,
    packets: VecDeque<Packet>,
}

impl Firewall {
    fn reset(&mut self) {
        self.t = 0;
        for scanner in self.scanners.iter_mut() {
            scanner.reset();
        }
    }

    fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    fn step(&mut self) -> Option<Packet> {
        let mut result = None;

        for packet in self.packets.iter_mut() {
            if packet.online {
                packet.step();
            } else {
                packet.online = true;
            }

            for scanner in &self.scanners {
                if packet.pos == scanner.depth && scanner.pos == 0 {
                    packet.severity += scanner.severity();
                    packet.caught = true;
                }
            }
        }

        if let Some(packet) = self.packets.pop_back() {
            if packet.pos < self.length {
                self.packets.push_back(packet);
            } else {
                result = Some(packet);
            }
        }

        for scanner in self.scanners.iter_mut() {
            scanner.step();
        }

        self.t += 1;
        result
    }

    fn add_packet(&mut self, packet: Packet) {
        self.packets.push_front(packet);
    }

    fn first_safe_t(&self) -> usize {
        let mut t = 0;
        loop {
            let mut safe = true;

            for scanner in &self.scanners {
                let cycle_len = 2 * (scanner.range - 1);
                let pos_at_t = (t + scanner.depth) % cycle_len;

                if pos_at_t == 0 {
                    safe = false;
                    break;
                }
            }

            if safe {
                return t;
            }
            t += 1;
        }
    }
}

impl From<&str> for Firewall {
    fn from(value: &str) -> Self {
        let mut scanners: Vec<Scanner> = value.lines().map(|line| Scanner::from(line)).collect();
        scanners.sort_by(|a, b| a.depth.cmp(&b.depth));
        let length = scanners.last().unwrap().depth + 1;

        Self {
            t: 0,
            length,
            scanners,
            packets: VecDeque::new(),
        }
    }
}

impl Display for Firewall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for depth in 0..self.length {
            write!(f, "{depth:^3} ").unwrap();

            let has_packet = self.packets.iter().filter(|p| p.pos == depth).count() > 0;
            let scanner = self.scanners.iter().filter(|s| s.depth == depth).next();

            if let Some(scanner) = scanner {
                if has_packet && scanner.pos == 0 {
                    write!(f, "(S)").unwrap();
                } else if has_packet {
                    write!(f, "( )").unwrap();
                } else if scanner.pos == 0 {
                    write!(f, "[S]").unwrap();
                } else {
                    write!(f, "[ ]").unwrap();
                }
                for r_pos in 1..scanner.range {
                    if scanner.pos == r_pos {
                        write!(f, "[S]").unwrap();
                    } else {
                        write!(f, "[ ]").unwrap();
                    }
                }
            } else if has_packet {
                write!(f, "(.) ").unwrap();
            } else {
                write!(f, "... ").unwrap();
            }

            writeln!(f, "").unwrap();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Firewall, Packet};

    #[test]
    fn test_examples() {
        let firewall = "\
            0: 3\n\
            1: 2\n\
            4: 4\n\
            6: 4\
        ";

        let mut firewall = Firewall::from(firewall);
        #[allow(unused_assignments)]
        let mut packet = None;

        firewall.add_packet(Packet::new(firewall.t));

        loop {
            if let Some(out) = firewall.step() {
                packet = Some(out);
                break;
            }
        }

        let packet = packet.unwrap();
        assert_eq!(packet.caught, true);
        assert_eq!(packet.severity, 24);
    }

    #[test]
    fn test_examples_part2() {
        let firewall = "\
            0: 3\n\
            1: 2\n\
            4: 4\n\
            6: 4\
        ";

        let mut firewall = Firewall::from(firewall);
        #[allow(unused_assignments)]
        let mut packet = None;

        let delay = firewall.first_safe_t();
        assert_eq!(delay, 10);

        for _ in 0..delay {
            firewall.step();
        }

        firewall.add_packet(Packet::new(firewall.t));

        loop {
            if let Some(out) = firewall.step() {
                if !out.caught {
                    packet = Some(out);
                    break;
                }
            }
        }

        let packet = packet.unwrap();
        assert_eq!(packet.caught, false);
        assert_eq!(packet.severity, 0);
        assert_eq!(packet.start_t, 10);
    }

    #[test]
    fn test_input() {
        let firewall = std::fs::read_to_string("input/firewall.txt").unwrap();

        let mut firewall = Firewall::from(firewall.as_str());
        #[allow(unused_assignments)]
        let mut packet = None;

        firewall.add_packet(Packet::new(firewall.t));

        loop {
            if let Some(out) = firewall.step() {
                packet = Some(out);
                break;
            }
        }

        let packet = packet.unwrap();
        assert_eq!(packet.caught, true);
        assert_eq!(packet.severity, 2164);
    }

    #[test]
    fn test_input_part2() {
        let firewall = std::fs::read_to_string("input/firewall.txt").unwrap();

        let mut firewall = Firewall::from(firewall.as_str());
        #[allow(unused_assignments)]
        let mut packet = None;

        let delay = firewall.first_safe_t();
        assert_eq!(delay, 3861798);

        for _ in 0..delay {
            firewall.step();
        }

        firewall.add_packet(Packet::new(firewall.t));
        loop {
            if let Some(out) = firewall.step() {
                if !out.caught {
                    packet = Some(out);
                    break;
                }
            }
        }

        let packet = packet.unwrap();
        assert_eq!(packet.caught, false);
        assert_eq!(packet.severity, 0);
        assert_eq!(packet.start_t, 3861798);
    }
}
