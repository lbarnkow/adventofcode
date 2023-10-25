#![allow(dead_code)]

use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2019 - day 12");
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Vec2d {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec2d {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::AddAssign<Vec2d> for Vec2d {
    fn add_assign(&mut self, rhs: Vec2d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Moon {
    pos: Vec2d,
    vel: Vec2d,
}

impl From<&str> for Moon {
    fn from(value: &str) -> Self {
        let caps = RE.captures(value).unwrap();
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let z = caps[3].parse().unwrap();
        Self::new(Vec2d::new(x, y, z), None)
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<x={:>3}, y={:>3}, z={:>3}>",
            self.pos.x, self.pos.y, self.pos.z, self.vel.x, self.vel.y, self.vel.z,
        )
    }
}

impl Moon {
    fn new(pos: Vec2d, vel: Option<Vec2d>) -> Self {
        Self {
            pos,
            vel: vel.unwrap_or_default(),
        }
    }

    fn update_velocity(&mut self, other: &Moon) {
        self.vel.x += match self.pos.x.cmp(&other.pos.x) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        self.vel.y += match self.pos.y.cmp(&other.pos.y) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        self.vel.z += match self.pos.z.cmp(&other.pos.z) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
    }

    fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }

    fn potential_energy(&self) -> usize {
        self.pos.x.unsigned_abs() + self.pos.y.unsigned_abs() + self.pos.z.unsigned_abs()
    }

    fn kinetic_energy(&self) -> usize {
        self.vel.x.unsigned_abs() + self.vel.y.unsigned_abs() + self.vel.z.unsigned_abs()
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct System {
    moons: Vec<Moon>,
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let moons = value.lines().map(|l| l.into()).collect();
        Self { moons }
    }
}

impl System {
    fn step(&mut self) {
        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                if i == j {
                    continue;
                }
                let other = &self.moons[j].clone();
                self.moons[i].update_velocity(other);
            }
        }

        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn print_moons(&self) -> String {
        self.moons
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn total_energ(&self) -> usize {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::System;

    // https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
    fn greatest_common_divisor(numbers: &[usize]) -> usize {
        let mut a = numbers[0];
        let mut b = if numbers.len() == 2 {
            numbers[1]
        } else {
            greatest_common_divisor(&numbers[1..])
        };

        if a > b {
            let t = a;
            a = b;
            b = t;
        }

        if a == 0 {
            return b;
        }

        greatest_common_divisor(&[a, b % a])
    }

    // https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor
    fn least_common_multiple(numbers: &[usize]) -> usize {
        let a = numbers[0];
        let b = if numbers.len() == 2 {
            numbers[1]
        } else {
            least_common_multiple(&numbers[1..])
        };

        a * (b / greatest_common_divisor(&[a, b]))
    }

    fn split_by_axis(
        system: &System,
    ) -> (
        Vec<(isize, isize)>,
        Vec<(isize, isize)>,
        Vec<(isize, isize)>,
    ) {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();
        for moon in &system.moons {
            x.push((moon.pos.x, moon.vel.x));
            y.push((moon.pos.y, moon.vel.y));
            z.push((moon.pos.z, moon.vel.z));
        }
        (x, y, z)
    }

    fn look_for_repeats(system: System) -> usize {
        let mut system = system;
        let (x_base, y_base, z_base) = split_by_axis(&system);

        let mut n = 0;
        let mut periods: [Option<usize>; 3] = [None; 3];

        loop {
            n += 1;
            system.step();
            let (x, y, z) = split_by_axis(&system);

            if periods[0].is_none() && x == x_base {
                periods[0] = Some(n);
            }
            if periods[1].is_none() && y == y_base {
                periods[1] = Some(n);
            }
            if periods[2].is_none() && z == z_base {
                periods[2] = Some(n);
            }

            if periods.iter().filter(|p| p.is_some()).count() == 3 {
                break;
            }
        }

        let (px, py, pz) = (
            periods[0].unwrap(),
            periods[1].unwrap(),
            periods[2].unwrap(),
        );
        least_common_multiple(&[px, py, pz])
    }

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(greatest_common_divisor(&[54, 24]), 6);
        assert_eq!(greatest_common_divisor(&[42, 56]), 14);
        assert_eq!(greatest_common_divisor(&[48, 18]), 6);
    }

    #[test]
    fn test_least_common_multiple() {
        assert_eq!(least_common_multiple(&[3, 4, 6]), 12);
        assert_eq!(least_common_multiple(&[48, 180]), 720);
        assert_eq!(least_common_multiple(&[8, 9, 21]), 504);
    }

    #[test]
    fn test_example_1() {
        let moons = "\
            <x=-1, y=0, z=2>\n\
            <x=2, y=-10, z=-7>\n\
            <x=4, y=-8, z=8>\n\
            <x=3, y=5, z=-1>\
        ";
        let mut system = System::from(moons);

        let expected = "\
            pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  2, y= -1, z=  1>, vel=<x=  3, y= -1, z= -1>\n\
            pos=<x=  3, y= -7, z= -4>, vel=<x=  1, y=  3, z=  3>\n\
            pos=<x=  1, y= -7, z=  5>, vel=<x= -3, y=  1, z= -3>\n\
            pos=<x=  2, y=  2, z=  0>, vel=<x= -1, y= -3, z=  1>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  5, y= -3, z= -1>, vel=<x=  3, y= -2, z= -2>\n\
            pos=<x=  1, y= -2, z=  2>, vel=<x= -2, y=  5, z=  6>\n\
            pos=<x=  1, y= -4, z= -1>, vel=<x=  0, y=  3, z= -6>\n\
            pos=<x=  1, y= -4, z=  2>, vel=<x= -1, y= -6, z=  2>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  5, y= -6, z= -1>, vel=<x=  0, y= -3, z=  0>\n\
            pos=<x=  0, y=  0, z=  6>, vel=<x= -1, y=  2, z=  4>\n\
            pos=<x=  2, y=  1, z= -5>, vel=<x=  1, y=  5, z= -4>\n\
            pos=<x=  1, y= -8, z=  2>, vel=<x=  0, y= -4, z=  0>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  2, y= -8, z=  0>, vel=<x= -3, y= -2, z=  1>\n\
            pos=<x=  2, y=  1, z=  7>, vel=<x=  2, y=  1, z=  1>\n\
            pos=<x=  2, y=  3, z= -6>, vel=<x=  0, y=  2, z= -1>\n\
            pos=<x=  2, y= -9, z=  1>, vel=<x=  1, y= -1, z= -1>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x= -1, y= -9, z=  2>, vel=<x= -3, y= -1, z=  2>\n\
            pos=<x=  4, y=  1, z=  5>, vel=<x=  2, y=  0, z= -2>\n\
            pos=<x=  2, y=  2, z= -4>, vel=<x=  0, y= -1, z=  2>\n\
            pos=<x=  3, y= -7, z= -1>, vel=<x=  1, y=  2, z= -2>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x= -1, y= -7, z=  3>, vel=<x=  0, y=  2, z=  1>\n\
            pos=<x=  3, y=  0, z=  0>, vel=<x= -1, y= -1, z= -5>\n\
            pos=<x=  3, y= -2, z=  1>, vel=<x=  1, y= -4, z=  5>\n\
            pos=<x=  3, y= -4, z= -2>, vel=<x=  0, y=  3, z= -1>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  2, y= -2, z=  1>, vel=<x=  3, y=  5, z= -2>\n\
            pos=<x=  1, y= -4, z= -4>, vel=<x= -2, y= -4, z= -4>\n\
            pos=<x=  3, y= -7, z=  5>, vel=<x=  0, y= -5, z=  4>\n\
            pos=<x=  2, y=  0, z=  0>, vel=<x= -1, y=  4, z=  2>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  5, y=  2, z= -2>, vel=<x=  3, y=  4, z= -3>\n\
            pos=<x=  2, y= -7, z= -5>, vel=<x=  1, y= -3, z= -1>\n\
            pos=<x=  0, y= -9, z=  6>, vel=<x= -3, y= -2, z=  1>\n\
            pos=<x=  1, y=  1, z=  3>, vel=<x= -1, y=  1, z=  3>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  5, y=  3, z= -4>, vel=<x=  0, y=  1, z= -2>\n\
            pos=<x=  2, y= -9, z= -3>, vel=<x=  0, y= -2, z=  2>\n\
            pos=<x=  0, y= -8, z=  4>, vel=<x=  0, y=  1, z= -2>\n\
            pos=<x=  1, y=  1, z=  5>, vel=<x=  0, y=  0, z=  2>\
        ";
        assert_eq!(system.print_moons(), expected);

        system.step();
        let expected = "\
            pos=<x=  2, y=  1, z= -3>, vel=<x= -3, y= -2, z=  1>\n\
            pos=<x=  1, y= -8, z=  0>, vel=<x= -1, y=  1, z=  3>\n\
            pos=<x=  3, y= -6, z=  1>, vel=<x=  3, y=  2, z= -3>\n\
            pos=<x=  2, y=  0, z=  4>, vel=<x=  1, y= -1, z= -1>\
        ";
        assert_eq!(system.print_moons(), expected);
        assert_eq!(system.total_energ(), 179);

        let system = System::from(moons);
        assert_eq!(look_for_repeats(system), 2772);
    }

    #[test]
    fn test_example_2() {
        // Example 2
        let moons = "\
            <x=-8, y=-10, z=0>\n\
            <x=5, y=5, z=10>\n\
            <x=2, y=-7, z=3>\n\
            <x=9, y=-8, z=-3>\
        ";
        let mut system = System::from(moons);

        let expected = "\
            pos=<x= -8, y=-10, z=  0>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  5, y=  5, z= 10>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  2, y= -7, z=  3>, vel=<x=  0, y=  0, z=  0>\n\
            pos=<x=  9, y= -8, z= -3>, vel=<x=  0, y=  0, z=  0>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x= -9, y=-10, z=  1>, vel=<x= -2, y= -2, z= -1>\n\
            pos=<x=  4, y= 10, z=  9>, vel=<x= -3, y=  7, z= -2>\n\
            pos=<x=  8, y=-10, z= -3>, vel=<x=  5, y= -1, z= -2>\n\
            pos=<x=  5, y=-10, z=  3>, vel=<x=  0, y= -4, z=  5>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x=-10, y=  3, z= -4>, vel=<x= -5, y=  2, z=  0>\n\
            pos=<x=  5, y=-25, z=  6>, vel=<x=  1, y=  1, z= -4>\n\
            pos=<x= 13, y=  1, z=  1>, vel=<x=  5, y= -2, z=  2>\n\
            pos=<x=  0, y=  1, z=  7>, vel=<x= -1, y= -1, z=  2>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x= 15, y= -6, z= -9>, vel=<x= -5, y=  4, z=  0>\n\
            pos=<x= -4, y=-11, z=  3>, vel=<x= -3, y=-10, z=  0>\n\
            pos=<x=  0, y= -1, z= 11>, vel=<x=  7, y=  4, z=  3>\n\
            pos=<x= -3, y= -2, z=  5>, vel=<x=  1, y=  2, z= -3>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x= 14, y=-12, z= -4>, vel=<x= 11, y=  3, z=  0>\n\
            pos=<x= -1, y= 18, z=  8>, vel=<x= -5, y=  2, z=  3>\n\
            pos=<x= -5, y=-14, z=  8>, vel=<x=  1, y= -2, z=  0>\n\
            pos=<x=  0, y=-12, z= -2>, vel=<x= -7, y= -3, z= -3>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x=-23, y=  4, z=  1>, vel=<x= -7, y= -1, z=  2>\n\
            pos=<x= 20, y=-31, z= 13>, vel=<x=  5, y=  3, z=  4>\n\
            pos=<x= -4, y=  6, z=  1>, vel=<x= -1, y=  1, z= -3>\n\
            pos=<x= 15, y=  1, z= -5>, vel=<x=  3, y= -3, z= -3>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x= 36, y=-10, z=  6>, vel=<x=  5, y=  0, z=  3>\n\
            pos=<x=-18, y= 10, z=  9>, vel=<x= -3, y= -7, z=  5>\n\
            pos=<x=  8, y=-12, z= -3>, vel=<x= -2, y=  1, z= -7>\n\
            pos=<x=-18, y= -8, z= -2>, vel=<x=  0, y=  6, z= -1>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x=-33, y= -6, z=  5>, vel=<x= -5, y= -4, z=  7>\n\
            pos=<x= 13, y= -9, z=  2>, vel=<x= -2, y= 11, z=  3>\n\
            pos=<x= 11, y= -8, z=  2>, vel=<x=  8, y= -6, z= -7>\n\
            pos=<x= 17, y=  3, z=  1>, vel=<x= -1, y= -1, z= -3>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x= 30, y= -8, z=  3>, vel=<x=  3, y=  3, z=  0>\n\
            pos=<x= -2, y= -4, z=  0>, vel=<x=  4, y=-13, z=  2>\n\
            pos=<x=-18, y= -7, z= 15>, vel=<x= -8, y=  2, z= -2>\n\
            pos=<x= -2, y= -1, z= -8>, vel=<x=  1, y=  8, z=  0>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x=-25, y= -1, z=  4>, vel=<x=  1, y= -3, z=  4>\n\
            pos=<x=  2, y= -9, z=  0>, vel=<x= -3, y= 13, z= -1>\n\
            pos=<x= 32, y= -8, z= 14>, vel=<x=  5, y= -4, z=  6>\n\
            pos=<x= -1, y= -2, z= -8>, vel=<x= -3, y= -6, z= -9>\
        ";
        assert_eq!(system.print_moons(), expected);

        (0..10).for_each(|_| system.step());
        let expected = "\
            pos=<x=  8, y=-12, z= -9>, vel=<x= -7, y=  3, z=  0>\n\
            pos=<x= 13, y= 16, z= -3>, vel=<x=  3, y=-11, z= -5>\n\
            pos=<x=-29, y=-11, z= -1>, vel=<x= -3, y=  7, z=  4>\n\
            pos=<x= 16, y=-13, z= 23>, vel=<x=  7, y=  1, z=  1>\
        ";
        assert_eq!(system.print_moons(), expected);
        assert_eq!(system.total_energ(), 1940);

        let system = System::from(moons);
        assert_eq!(look_for_repeats(system), 4_686_774_924);
    }

    #[test]
    fn test_input() {
        let moons = std::fs::read_to_string("input/moons.txt").unwrap();
        let mut system = System::from(moons.as_str());

        (0..1000).for_each(|_| system.step());
        assert_eq!(system.total_energ(), 10189);

        let system = System::from(moons.as_str());
        assert_eq!(look_for_repeats(system), 469_671_086_427_712);
    }
}
