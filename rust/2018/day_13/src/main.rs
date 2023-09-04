#![allow(dead_code)]

use std::{cmp::Ordering, collections::HashMap, fmt::Display};

fn main() {
    println!("Advent of Code 2018 - day 13");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Segment {
    Empty,
    Vertical,
    Horizontal,
    CurveSlash,
    CurveBackslash,
    Intersection,
}

impl From<char> for Segment {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Empty,
            '|' | 'v' | '^' => Self::Vertical,
            '-' | '<' | '>' => Self::Horizontal,
            '/' => Self::CurveSlash,
            '\\' => Self::CurveBackslash,
            '+' => Self::Intersection,
            _ => panic!("Illegal track char: {value}!"),
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => ' ',
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::CurveSlash => '/',
            Self::CurveBackslash => '\\',
            Self::Intersection => '+',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn parse(value: char) -> Option<Self> {
        match value {
            '^' => Some(Self::N),
            '>' => Some(Self::E),
            'v' => Some(Self::S),
            '<' => Some(Self::W),
            _ => None,
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::N => '^',
            Dir::E => '>',
            Dir::S => 'v',
            Dir::W => '<',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn offset(&self, dir: Dir) -> Self {
        let mut p = *self;
        match dir {
            Dir::N => p.y -= 1,
            Dir::E => p.x += 1,
            Dir::S => p.y += 1,
            Dir::W => p.x -= 1,
        };
        p
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CartMoves {
    Left,
    Straight,
    Right,
}

impl CartMoves {
    fn update(&self) -> Self {
        match self {
            CartMoves::Left => Self::Straight,
            CartMoves::Straight => Self::Right,
            CartMoves::Right => Self::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cart {
    pos: Pos,
    dir: Dir,
    next: CartMoves,
}

impl Cart {
    fn new(pos: Pos, dir: Dir, next: Option<CartMoves>) -> Self {
        Self {
            pos,
            dir,
            next: match next {
                Some(next) => next,
                None => CartMoves::Left,
            },
        }
    }

    fn move_to(&mut self, next_pos: Pos, segment: Segment) {
        match (&self.dir, segment) {
            (_, Segment::Empty) => panic!(
                "Illegal move for cart {:?} into new pos {:?} with segment '{}'!",
                self, &next_pos, segment
            ),
            (Dir::N, Segment::Horizontal)
            | (Dir::E, Segment::Vertical)
            | (Dir::S, Segment::Horizontal)
            | (Dir::W, Segment::Vertical) => panic!(
                "Illegal move for cart {:?} into new pos {:?} with segment {}!",
                self, &next_pos, segment
            ),
            _ => self.pos = next_pos,
        }
        match segment {
            Segment::CurveSlash | Segment::CurveBackslash | Segment::Intersection => {
                self.turn(segment)
            }
            _ => (),
        }
    }

    fn turn(&mut self, segment: Segment) {
        self.dir = match (segment, &self.next) {
            (Segment::Intersection, CartMoves::Straight) => self.dir,
            (Segment::Intersection, CartMoves::Left) => match self.dir {
                Dir::N => Dir::W,
                Dir::E => Dir::N,
                Dir::S => Dir::E,
                Dir::W => Dir::S,
            },
            (Segment::Intersection, CartMoves::Right) => match self.dir {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            },
            (Segment::CurveSlash, _) => match self.dir {
                Dir::N => Dir::E,
                Dir::E => Dir::N,
                Dir::S => Dir::W,
                Dir::W => Dir::S,
            },
            (Segment::CurveBackslash, _) => match self.dir {
                Dir::N => Dir::W,
                Dir::E => Dir::S,
                Dir::S => Dir::E,
                Dir::W => Dir::N,
            },
            _ => panic!("Segment {} does not allow turning!", segment),
        };
        if segment == Segment::Intersection {
            self.next = self.next.update();
        }
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Equal => self.pos.x.cmp(&other.pos.x),
            ord => ord,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Track {
    t: usize,
    width: usize,
    height: usize,
    segments: Vec<Segment>,
    carts: Vec<Cart>,
    collisions: HashMap<usize, Vec<Pos>>,
}

impl From<&str> for Track {
    fn from(value: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut segments = Vec::new();
        let mut carts = Vec::new();

        for (y, line) in value.lines().enumerate() {
            height += 1;
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                segments.push(c.into());
                if let Some(dir) = Dir::parse(c) {
                    carts.push(Cart::new(Pos::new(x, y), dir, None));
                }
            }
        }
        carts.sort();

        Self {
            t: 0,
            width,
            height,
            segments,
            carts,
            collisions: HashMap::new(),
        }
    }
}

impl Track {
    fn idx(width: usize, pos: &Pos) -> usize {
        pos.y * width + pos.x
    }

    fn tick(&mut self) {
        self.t += 1;

        self.carts.sort();
        let mut collisions = Vec::new();
        let mut cart_idx = 0;
        while cart_idx < self.carts.len() {
            let cart = &mut self.carts[cart_idx];
            let next_pos = cart.pos.offset(cart.dir);
            cart.move_to(next_pos, self.segments[Track::idx(self.width, &next_pos)]);

            let mut carts_at_pos = self
                .carts
                .iter()
                .enumerate()
                .filter(|(_, c)| next_pos == c.pos)
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>();

            if carts_at_pos.len() > 1 {
                carts_at_pos.sort();
                carts_at_pos.into_iter().rev().for_each(|idx| {
                    self.carts.remove(idx);
                    if idx < cart_idx {
                        cart_idx -= 1;
                    }
                });
                collisions.push(next_pos);
            } else {
                cart_idx += 1;
            }
        }
        self.collisions.insert(self.t, collisions);

        self.carts.sort();
    }

    fn collisions(&self, t: usize) -> &[Pos] {
        if let Some(collisions) = self.collisions.get(&t) {
            collisions
        } else {
            &[]
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x, y);
                let idx = Track::idx(self.width, &pos);
                let collisions = &self.collisions.get(&self.t);
                if collisions.is_some() && collisions.unwrap().contains(&pos) {
                    write!(f, "X").unwrap();
                } else {
                    match self.carts.iter().find(|c| c.pos == pos) {
                        Some(cart) => write!(f, "{}", cart.dir).unwrap(),
                        None => write!(f, "{}", &self.segments[idx]).unwrap(),
                    };
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cart, CartMoves, Dir, Pos, Track};

    #[test]
    fn test_example_1() {
        let map = std::fs::read_to_string("input/example1.txt").unwrap();
        let mut track = Track::from(map.as_str());
        assert_eq!(track.t, 0);
        assert_eq!(track.collisions(0), &[]);

        track.tick();
        assert_eq!(track.t, 1);
        assert_eq!(track.collisions(1), &[]);

        track.tick();
        assert_eq!(track.t, 2);
        assert_eq!(track.collisions(2), &[Pos::new(0, 3)]);
    }

    #[test]
    fn test_example_2() {
        let map = std::fs::read_to_string("input/example2.txt").unwrap();
        let mut track = Track::from(map.as_str());
        assert_eq!(track.t, 0);
        assert_eq!(track.collisions(0), &[]);
        assert_eq!(
            track.carts,
            &[
                Cart::new(Pos::new(2, 0), Dir::E, Some(CartMoves::Left)),
                Cart::new(Pos::new(5, 1), Dir::S, Some(CartMoves::Left)),
                Cart::new(Pos::new(0, 2), Dir::N, Some(CartMoves::Left)),
                Cart::new(Pos::new(3, 3), Dir::W, Some(CartMoves::Left)),
            ]
        );

        track.tick();
        assert_eq!(track.t, 1);
        assert_eq!(track.collisions(1), &[]);
        assert_eq!(
            track.carts,
            &[
                Cart::new(Pos::new(3, 0), Dir::E, Some(CartMoves::Left)),
                Cart::new(Pos::new(0, 1), Dir::N, Some(CartMoves::Left)),
                Cart::new(Pos::new(5, 2), Dir::S, Some(CartMoves::Left)),
                Cart::new(Pos::new(2, 3), Dir::W, Some(CartMoves::Left)),
            ]
        );

        track.tick();
        assert_eq!(track.t, 2);
        assert_eq!(track.collisions(2), &[]);
        assert_eq!(
            track.carts,
            &[
                Cart::new(Pos::new(0, 0), Dir::E, Some(CartMoves::Left)),
                Cart::new(Pos::new(4, 0), Dir::E, Some(CartMoves::Left)),
                Cart::new(Pos::new(1, 3), Dir::W, Some(CartMoves::Left)),
                Cart::new(Pos::new(5, 3), Dir::W, Some(CartMoves::Left)),
            ]
        );

        track.tick();
        assert_eq!(track.t, 3);
        assert_eq!(track.collisions(3), &[]);
        assert_eq!(
            track.carts,
            &[
                Cart::new(Pos::new(1, 0), Dir::E, Some(CartMoves::Left)),
                Cart::new(Pos::new(5, 0), Dir::S, Some(CartMoves::Left)),
                Cart::new(Pos::new(0, 3), Dir::N, Some(CartMoves::Left)),
                Cart::new(Pos::new(4, 3), Dir::W, Some(CartMoves::Left)),
            ]
        );
    }

    #[test]
    fn test_example_3() {
        static EXPECTED: [&str; 15] = [
            "/->-\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/-->\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | |  |\n\\-+-/  \\->--/\n  \\------/   \n",
            "/---v        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | |  |\n\\-+-/  \\-+>-/\n  \\------/   \n",
            "/---\\        \n|   v  /----\\\n| /-+--+-\\  |\n| | |  | |  |\n\\-+-/  \\-+->/\n  \\------/   \n",
            "/---\\        \n|   |  /----\\\n| /->--+-\\  |\n| | |  | |  |\n\\-+-/  \\-+--^\n  \\------/   \n",
            "/---\\        \n|   |  /----\\\n| /-+>-+-\\  |\n| | |  | |  ^\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /----\\\n| /-+->+-\\  ^\n| | |  | |  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /----<\n| /-+-->-\\  |\n| | |  | |  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /---<\\\n| /-+--+>\\  |\n| | |  | |  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /--<-\\\n| /-+--+-v  |\n| | |  | |  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /-<--\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /<---\\\n| /-+--+-\\  |\n| | |  | |  |\n\\-+-/  \\-<--/\n  \\------/   \n",
            "/---\\        \n|   |  v----\\\n| /-+--+-\\  |\n| | |  | |  |\n\\-+-/  \\<+--/\n  \\------/   \n",
            "/---\\        \n|   |  /----\\\n| /-+--v-\\  |\n| | |  | |  |\n\\-+-/  ^-+--/\n  \\------/   \n",
            "/---\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  X |  |\n\\-+-/  \\-+--/\n  \\------/   \n",
        ];

        let map = std::fs::read_to_string("input/example3.txt").unwrap();
        let mut track = Track::from(map.as_str());
        assert_eq!(track.t, 0);
        assert_eq!(track.collisions(0), &[]);
        assert_eq!(track.to_string(), EXPECTED[0]);

        for t in 1..EXPECTED.len() - 1 {
            track.tick();
            assert_eq!(track.t, t);
            assert_eq!(track.collisions(t), &[]);
            assert_eq!(track.to_string(), EXPECTED[t]);
        }

        track.tick();
        assert_eq!(track.t, EXPECTED.len() - 1);
        assert_eq!(track.collisions(EXPECTED.len() - 1), &[Pos::new(7, 3)]);
        assert_eq!(track.to_string(), *EXPECTED.last().unwrap());
    }

    #[test]
    fn test_example_4() {
        let map = std::fs::read_to_string("input/example4.txt").unwrap();
        let mut track = Track::from(map.as_str());

        while track.collisions(track.t).is_empty() {
            track.tick();
        }

        assert_eq!(track.t, 5);
        assert_eq!(track.collisions(track.t), [Pos::new(0, 1)]);
    }

    #[test]
    fn test_example1_part2() {
        static EXPECTED: [&str; 4] = [
            "/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/\n",
            "/---\\  \n|   |  \n| v-+-\\\n| | | |\n\\-+-/ |\n  |   |\n  ^---^\n",
            "/---\\  \n|   |  \n| /-+-\\\n| v | |\n\\-+-/ |\n  ^   ^\n  \\---/\n",
            "/---\\  \n|   |  \n| /-+-\\\n| | | |\n\\-+-/ ^\n  |   |\n  \\---/\n",
        ];

        let map = std::fs::read_to_string("input/example1_part2.txt").unwrap();
        let mut track = Track::from(map.as_str());
        assert_eq!(track.t, 0);
        assert_eq!(track.collisions(0), &[]);
        assert_eq!(track.to_string(), EXPECTED[0]);

        println!("{track}");
        for t in 1..EXPECTED.len() - 1 {
            track.tick();
            track.collisions.clear();
            assert_eq!(track.t, t);
            assert_eq!(track.to_string(), EXPECTED[t]);
        }

        track.tick();
        track.collisions.clear();
        assert_eq!(track.t, EXPECTED.len() - 1);
        assert_eq!(track.to_string(), *EXPECTED.last().unwrap());
    }

    #[test]
    fn test_input() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let mut track = Track::from(map.as_str());

        while track.collisions(track.t).is_empty() {
            track.tick();
        }

        assert_eq!(track.t, 299);
        assert_eq!(track.collisions(track.t).len(), 1);
        assert_eq!(track.collisions(track.t).first(), Some(&Pos::new(124, 130)));
    }

    #[test]
    fn test_input_part2() {
        let map = std::fs::read_to_string("input/map.txt").unwrap();
        let mut track = Track::from(map.as_str());

        while track.carts.len() > 1 {
            track.tick();
            track.collisions.clear();
        }

        assert_eq!(track.t, 19149);
        assert_eq!(track.carts.first().unwrap().pos, Pos::new(143, 123));
    }
}
