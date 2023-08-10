#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 11");
}

#[derive(Debug, Default, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn go(&self, dir: Dir) -> Self {
        let (x_offset, y_offset) = match dir {
            Dir::NW => (-1, -1),
            Dir::N => (0, -2),
            Dir::NE => (1, -1),
            Dir::SE => (1, 1),
            Dir::S => (0, 2),
            Dir::SW => (-1, 1),
        };
        Self {
            x: self.x + x_offset,
            y: self.y + y_offset,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    NW,
    N,
    NE,
    SE,
    S,
    SW,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "nw" => Self::NW,
            "n" => Self::N,
            "ne" => Self::NE,
            "se" => Self::SE,
            "s" => Self::S,
            "sw" => Self::SW,
            _ => panic!("Illegal dir: '{value}'!"),
        }
    }
}

impl Dir {
    fn from_list(s: &str) -> Vec<Self> {
        s.split(",").map(|dir| Dir::from(dir)).collect()
    }
}

#[derive(Debug, Default)]
struct Stats {
    final_dist: usize,
    max_dist: usize,
}

fn distance(mut pos: Pos) -> usize {
    pos.x = pos.x.abs();
    pos.y = pos.y.abs();

    let dist = pos.x;
    for _ in 0..dist {
        if pos.y > 0 {
            pos.y -= 1;
        } else {
            pos.y += 1;
        }
    }
    let dist = dist + (pos.y / 2);

    dist.try_into().unwrap()
}

fn retrace_steps(dirs: &Vec<Dir>) -> Stats {
    let mut pos = Pos::default();
    let mut stats = Stats::default();

    for dir in dirs {
        pos = pos.go(*dir);
        stats.max_dist = stats.max_dist.max(distance(pos));
    }

    stats.final_dist = distance(pos);
    stats
}

#[cfg(test)]
mod tests {
    use crate::{retrace_steps, Dir};

    #[test]
    fn test_examples() {
        let dirs = Dir::from_list("ne,ne,ne");
        let stats = retrace_steps(&dirs);
        assert_eq!(stats.final_dist, 3);
        assert_eq!(stats.max_dist, 3);

        let dirs = Dir::from_list("ne,ne,sw,sw");
        let stats = retrace_steps(&dirs);
        assert_eq!(stats.final_dist, 0);
        assert_eq!(stats.max_dist, 2);

        let dirs = Dir::from_list("ne,ne,s,s");
        let stats = retrace_steps(&dirs);
        assert_eq!(stats.final_dist, 2);
        assert_eq!(stats.max_dist, 2);

        let dirs = Dir::from_list("se,sw,se,sw,sw");
        let stats = retrace_steps(&dirs);
        assert_eq!(stats.final_dist, 3);
        assert_eq!(stats.max_dist, 3);
    }

    #[test]
    fn test_input() {
        let dirs = std::fs::read_to_string("input/dirs.txt").unwrap();
        let dirs = Dir::from_list(&dirs);
        let stats = retrace_steps(&dirs);
        assert_eq!(stats.final_dist, 650);
        assert_eq!(stats.max_dist, 1465);
    }
}
