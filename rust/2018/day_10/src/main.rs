#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 10");
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2d {
    x: i64,
    y: i64,
}

impl Vec2d {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vec2d {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug)]
struct Light {
    pos: Vec2d,
    vel: Vec2d,
}

impl From<&str> for Light {
    fn from(value: &str) -> Self {
        let caps = RE.captures(value).unwrap();

        Self {
            pos: Vec2d {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            },
            vel: Vec2d {
                x: caps[3].parse().unwrap(),
                y: caps[4].parse().unwrap(),
            },
        }
    }
}

fn parse_lights(lights: &str) -> Vec<Light> {
    lights.lines().map(|line| line.into()).collect()
}

fn compute_height(lights: &[Light]) -> i64 {
    let y_range = lights.iter().fold((i64::MAX, i64::MIN), |mut acc, light| {
        acc.0 = acc.0.min(light.pos.y);
        acc.1 = acc.1.max(light.pos.y);
        acc
    });

    y_range.1 - y_range.0 + 1
}

fn compute_width(lights: &[Light]) -> i64 {
    let y_range = lights.iter().fold((i64::MAX, i64::MIN), |mut acc, light| {
        acc.0 = acc.0.min(light.pos.x);
        acc.1 = acc.1.max(light.pos.x);
        acc
    });

    y_range.1 - y_range.0 + 1
}

fn render_lights(lights: &[Light]) -> String {
    let width = compute_width(lights);
    let height = compute_height(lights);

    let (min_x, min_y) = lights.iter().fold((i64::MAX, i64::MAX), |mut acc, light| {
        acc.0 = acc.0.min(light.pos.x);
        acc.1 = acc.1.min(light.pos.y);
        acc
    });

    let mut buf = vec!['.'; (width * height) as usize];
    for light in lights {
        let x = light.pos.x - min_x;
        let y = light.pos.y - min_y;
        let idx = (y * width + x) as usize;
        buf[idx] = '#';
    }

    let mut result = String::with_capacity(buf.len() + height as usize);
    let mut i = 0;
    for c in &buf {
        result.push(*c);
        i += 1;
        if i == width {
            result.push('\n');
            i = 0;
        }
    }
    result
}

fn simulate_to_smallest_height(lights: &mut [Light]) -> (i64, String) {
    let mut t = 0;
    let mut prev_height = compute_height(lights);

    loop {
        for light in lights.iter_mut() {
            light.pos += light.vel;
        }

        let height = compute_height(lights);
        if height > prev_height {
            break;
        }
        prev_height = height;
        t += 1;
    }

    for light in lights.iter_mut() {
        light.pos -= light.vel;
    }

    (t, render_lights(lights))
}

#[cfg(test)]
mod tests {
    use crate::{parse_lights, simulate_to_smallest_height};

    #[test]
    fn test_examples() {
        let lights = "\
            position=< 9,  1> velocity=< 0,  2>\n\
            position=< 7,  0> velocity=<-1,  0>\n\
            position=< 3, -2> velocity=<-1,  1>\n\
            position=< 6, 10> velocity=<-2, -1>\n\
            position=< 2, -4> velocity=< 2,  2>\n\
            position=<-6, 10> velocity=< 2, -2>\n\
            position=< 1,  8> velocity=< 1, -1>\n\
            position=< 1,  7> velocity=< 1,  0>\n\
            position=<-3, 11> velocity=< 1, -2>\n\
            position=< 7,  6> velocity=<-1, -1>\n\
            position=<-2,  3> velocity=< 1,  0>\n\
            position=<-4,  3> velocity=< 2,  0>\n\
            position=<10, -3> velocity=<-1,  1>\n\
            position=< 5, 11> velocity=< 1, -2>\n\
            position=< 4,  7> velocity=< 0, -1>\n\
            position=< 8, -2> velocity=< 0,  1>\n\
            position=<15,  0> velocity=<-2,  0>\n\
            position=< 1,  6> velocity=< 1,  0>\n\
            position=< 8,  9> velocity=< 0, -1>\n\
            position=< 3,  3> velocity=<-1,  1>\n\
            position=< 0,  5> velocity=< 0, -1>\n\
            position=<-2,  2> velocity=< 2,  0>\n\
            position=< 5, -2> velocity=< 1,  2>\n\
            position=< 1,  4> velocity=< 2,  1>\n\
            position=<-2,  7> velocity=< 2, -2>\n\
            position=< 3,  6> velocity=<-1, -1>\n\
            position=< 5,  0> velocity=< 1,  0>\n\
            position=<-6,  0> velocity=< 2,  0>\n\
            position=< 5,  9> velocity=< 1, -2>\n\
            position=<14,  7> velocity=<-2,  0>\n\
            position=<-3,  6> velocity=< 2, -1>\
        ";
        let mut lights = parse_lights(lights);

        let (t, msg) = simulate_to_smallest_height(&mut lights);
        let expected = "\
            #...#..###\n\
            #...#...#.\n\
            #...#...#.\n\
            #####...#.\n\
            #...#...#.\n\
            #...#...#.\n\
            #...#...#.\n\
            #...#..###\n\
        ";
        assert_eq!(t, 3);
        assert_eq!(msg, expected);
    }

    #[test]
    fn test_input() {
        let lights = std::fs::read_to_string("input/lights.txt").unwrap();
        let mut lights = parse_lights(&lights);

        let (t, msg) = simulate_to_smallest_height(&mut lights);
        let expected = "\
            #####...#####...#....#..#....#..#....#..######..######..#####.\n\
            #....#..#....#..##...#..##...#..#....#..#............#..#....#\n\
            #....#..#....#..##...#..##...#...#..#...#............#..#....#\n\
            #....#..#....#..#.#..#..#.#..#...#..#...#...........#...#....#\n\
            #####...#####...#.#..#..#.#..#....##....#####......#....#####.\n\
            #..#....#.......#..#.#..#..#.#....##....#.........#.....#..#..\n\
            #...#...#.......#..#.#..#..#.#...#..#...#........#......#...#.\n\
            #...#...#.......#...##..#...##...#..#...#.......#.......#...#.\n\
            #....#..#.......#...##..#...##..#....#..#.......#.......#....#\n\
            #....#..#.......#....#..#....#..#....#..#.......######..#....#\n\
        ";
        assert_eq!(t, 10946);
        assert_eq!(msg, expected);
    }
}
