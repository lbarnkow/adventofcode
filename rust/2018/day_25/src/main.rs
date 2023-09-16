#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};

fn main() {
    println!("Advent of Code 2018 - day 25");
}

#[derive(Clone, Copy)]
struct Point {
    coords: [isize; 4],
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(',').collect();

        let coords = [
            split[0].parse().unwrap(),
            split[1].parse().unwrap(),
            split[2].parse().unwrap(),
            split[3].parse().unwrap(),
        ];

        Self { coords }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.coords[0], self.coords[1], self.coords[2], self.coords[3],
        )
    }
}

impl Point {
    fn dist(&self, other: &Self) -> usize {
        self.coords[0].abs_diff(other.coords[0])
            + self.coords[1].abs_diff(other.coords[1])
            + self.coords[2].abs_diff(other.coords[2])
            + self.coords[3].abs_diff(other.coords[3])
    }
}

fn count_constellations(points: &[Point]) -> usize {
    let mut constellations: Vec<(Point, usize)> = points.iter().map(|p| (*p, 0)).collect();
    let mut max_num = 1;

    for i in 0..points.len() {
        let current = constellations[i].0;

        let mut my_constellation = max_num;
        constellations[i].1 = max_num;

        let mut to_merge = Vec::new();

        for (j, (other, other_constellation)) in constellations.iter_mut().enumerate() {
            if i == j {
                continue;
            }
            if current.dist(other) <= 3 {
                if *other_constellation == 0 {
                    *other_constellation = my_constellation;
                } else {
                    to_merge.push(my_constellation);
                    my_constellation = *other_constellation;
                }
            }
        }

        if !to_merge.is_empty() {
            for (_, constellation) in constellations.iter_mut() {
                if to_merge.contains(constellation) {
                    *constellation = my_constellation;
                }
            }
        }

        if my_constellation == max_num {
            max_num += 1;
        }
    }

    let constellations: HashSet<usize> = constellations
        .into_iter()
        .map(|(_, constellation)| constellation)
        .collect();

    constellations.len()
}

#[cfg(test)]
mod tests {
    use crate::{count_constellations, Point};

    #[test]
    fn test_example() {
        let points = "\
            0,0,0,0\n\
            3,0,0,0\n\
            0,3,0,0\n\
            0,0,3,0\n\
            0,0,0,3\n\
            0,0,0,6\n\
            9,0,0,0\n\
            12,0,0,0\
        ";
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 2);

        let points = "\
            6,0,0,0\n\
            0,0,0,0\n\
            3,0,0,0\n\
            0,3,0,0\n\
            0,0,3,0\n\
            0,0,0,3\n\
            0,0,0,6\n\
            9,0,0,0\n\
            12,0,0,0\
        ";
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 1);

        let points = "\
            -1,2,2,0\n\
            0,0,2,-2\n\
            0,0,0,-2\n\
            -1,2,0,0\n\
            -2,-2,-2,2\n\
            3,0,2,-1\n\
            -1,3,2,2\n\
            -1,0,-1,0\n\
            0,2,1,-2\n\
            3,0,0,0\
        ";
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 4);

        let points = "\
            1,-1,0,1\n\
            2,0,-1,0\n\
            3,2,-1,0\n\
            0,0,3,1\n\
            0,0,-1,-1\n\
            2,3,-2,0\n\
            -2,2,0,0\n\
            2,-2,0,-1\n\
            1,-1,0,-1\n\
            3,2,0,2\
        ";
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 3);

        let points = "\
            1,-1,-1,-2\n\
            -2,-2,0,1\n\
            0,2,1,3\n\
            -2,3,-2,1\n\
            0,2,3,-2\n\
            -1,-1,1,-2\n\
            0,-2,-1,0\n\
            -2,2,3,-1\n\
            1,2,2,0\n\
            -1,-2,0,-2\
        ";
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 8);
    }

    #[test]
    fn test_input() {
        let points = std::fs::read_to_string("input/points.txt").unwrap();
        let points: Vec<Point> = points.lines().map(|p| p.into()).collect();
        assert_eq!(count_constellations(&points), 375);
    }
}
