#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn main() {
    println!("Advent of Code 2018 - day 06");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl From<&str> for Coordinate {
    fn from(value: &str) -> Self {
        let mut iter = value.split(",");

        let x = iter.next().unwrap().trim().parse().unwrap();
        let y = iter.next().unwrap().trim().parse().unwrap();

        Self { x, y }
    }
}

impl Coordinate {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn from_listing(listing: &str) -> Vec<Self> {
        listing.lines().map(|line| line.into()).collect()
    }

    fn dist(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

fn compute_bounding_box(coords: &[Coordinate]) -> (Coordinate, Coordinate) {
    let min_max = coords
        .iter()
        .fold((i64::MAX, i64::MAX, i64::MIN, i64::MIN), |acc, c| {
            (
                acc.0.min(c.x),
                acc.1.min(c.y),
                acc.2.max(c.x),
                acc.3.max(c.y),
            )
        });

    (
        Coordinate::new(min_max.0, min_max.1),
        Coordinate::new(min_max.2, min_max.3),
    )
}

fn compute_closest(coords: &[Coordinate], poi: Coordinate) -> Option<Coordinate> {
    let mut closest = coords[0];
    let mut dist = closest.dist(&poi);
    let mut count = 1;

    for c in &coords[1..] {
        let tmp_dist = c.dist(&poi);
        if tmp_dist == dist {
            count += 1;
        } else if tmp_dist < dist {
            closest = *c;
            dist = tmp_dist;
            count = 1;
        }
    }

    if count == 1 {
        Some(closest)
    } else {
        None
    }
}

fn compute_areas(coords: &[Coordinate]) -> HashMap<Coordinate, u64> {
    let mut areas = HashMap::with_capacity(coords.len());
    let mut infinite = HashSet::with_capacity(coords.len());

    let bbox = compute_bounding_box(coords);

    for y in bbox.0.y..=bbox.1.y {
        for x in bbox.0.x..=bbox.1.x {
            if let Some(closest) = compute_closest(coords, Coordinate::new(x, y)) {
                if x == bbox.0.x || x == bbox.1.x || y == bbox.0.y || y == bbox.1.y {
                    infinite.insert(closest);
                    continue;
                }
                if let Some(area) = areas.get_mut(&closest) {
                    *area += 1;
                } else {
                    areas.insert(closest, 1);
                }
            }
        }
    }

    let areas = areas
        .into_iter()
        .filter(|(c, _)| !infinite.contains(c))
        .collect();
    areas
}

fn compute_safe_region(coords: &[Coordinate], max_dist: i64) -> usize {
    let mut area = 0;
    let bbox = compute_bounding_box(coords);

    for y in bbox.0.y..=bbox.1.y {
        for x in bbox.0.x..=bbox.1.x {
            let poi = Coordinate::new(x, y);
            let total_dist: i64 = coords.iter().map(|c| c.dist(&poi)).sum();

            if total_dist < max_dist {
                area += 1;
            }
        }
    }

    area
}

#[cfg(test)]
mod tests {
    use crate::{compute_areas, compute_safe_region, Coordinate};

    #[test]
    fn test_examples() {
        let coords = "\
            1, 1\n\
            1, 6\n\
            8, 3\n\
            3, 4\n\
            5, 5\n\
            8, 9\
        ";
        let coords = Coordinate::from_listing(coords);

        let areas = compute_areas(&coords);
        assert_eq!(areas.len(), 2);

        let largest = areas
            .into_iter()
            .max_by(|(_, a1), (_, a2)| a1.cmp(a2))
            .unwrap();

        assert_eq!(largest.0, Coordinate::new(5, 5));
        assert_eq!(largest.1, 17);

        let safe_region = compute_safe_region(&coords, 32);
        assert_eq!(safe_region, 16);
    }

    #[test]
    fn test_input() {
        let coords = std::fs::read_to_string("input/coords.txt").unwrap();
        let coords = Coordinate::from_listing(&coords);

        let areas = compute_areas(&coords);
        assert_eq!(areas.len(), 29);

        let largest = areas
            .into_iter()
            .max_by(|(_, a1), (_, a2)| a1.cmp(a2))
            .unwrap();

        assert_eq!(largest.0, Coordinate::new(150, 287));
        assert_eq!(largest.1, 3989);

        let safe_region = compute_safe_region(&coords, 10_000);
        assert_eq!(safe_region, 49_715);
    }
}
