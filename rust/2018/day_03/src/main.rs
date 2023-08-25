#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Debug;
use std::ops::Add;

fn main() {
    println!("Advent of Code 2018 - day 03");
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Vector2D<T>
where
    T: Debug + Copy + PartialEq + Add<Output = T>,
{
    x: T,
    y: T,
}

impl<T> Vector2D<T>
where
    T: Debug + Copy + PartialEq + Add<Output = T>,
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> std::ops::Add for Vector2D<T>
where
    T: Debug + Copy + PartialEq + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Claim {
    id: usize,
    pos: Vector2D<i64>,
    size: Vector2D<i64>,
}

impl From<&str> for Claim {
    fn from(value: &str) -> Self {
        let caps = RE.captures(value).unwrap();

        let id = caps[1].parse().unwrap();
        let pos = Vector2D::new(caps[2].parse().unwrap(), caps[3].parse().unwrap());
        let size = Vector2D::new(caps[4].parse().unwrap(), caps[5].parse().unwrap());

        Self { id, pos, size }
    }
}

impl Claim {
    fn overlaps(&self, rhs: &Self) -> bool {
        self.pos.x < rhs.pos.x + rhs.size.x
            && rhs.pos.x < self.pos.x + self.size.x
            && self.pos.y < rhs.pos.y + rhs.size.y
            && rhs.pos.y < self.pos.y + self.size.y
    }
}

#[derive(Debug)]
struct ClaimsInfo {
    claims: Vec<Claim>,
    total_size: Vector2D<i64>,
}

impl From<&str> for ClaimsInfo {
    fn from(value: &str) -> Self {
        let claims: Vec<Claim> = value.lines().map(|line| Claim::from(line)).collect();

        let total_size = (&claims)
            .iter()
            .fold(Vector2D::<i64>::default(), |mut acc, e| {
                acc.x = acc.x.max(e.pos.x + e.size.x);
                acc.y = acc.y.max(e.pos.y + e.size.y);
                acc
            });

        Self { claims, total_size }
    }
}

impl ClaimsInfo {
    fn count_squares_with_overlapping_claims(&self) -> usize {
        let width: usize = self.total_size.x.try_into().unwrap();
        let area = (self.total_size.x * self.total_size.y).try_into().unwrap();
        let mut map = vec![0; area];

        for claim in &self.claims {
            for x in 0..claim.size.x {
                for y in 0..claim.size.y {
                    let x: usize = (claim.pos.x + x).try_into().unwrap();
                    let y: usize = (claim.pos.y + y).try_into().unwrap();
                    map[y * width + x] += 1;
                }
            }
        }

        map.iter().filter(|e| **e > 1).count()
    }

    fn find_non_overlapping_claim(&self) -> &Claim {
        for claim in &self.claims {
            let mut overlap = false;
            for other in &self.claims {
                if claim.id == other.id {
                    continue;
                }
                if claim.overlaps(other) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                return claim;
            }
        }

        panic!("No non-overlapping claims found!");
    }
}

#[cfg(test)]
mod tests {
    use crate::ClaimsInfo;

    #[test]
    fn test_examples() {
        let claims = "\
            #1 @ 1,3: 4x4\n\
            #2 @ 3,1: 4x4\n\
            #3 @ 5,5: 2x2\
        ";
        let claims = ClaimsInfo::from(claims);

        assert_eq!(claims.total_size.x, 7);
        assert_eq!(claims.total_size.y, 7);
        assert_eq!(claims.count_squares_with_overlapping_claims(), 4);

        assert_eq!(claims.find_non_overlapping_claim().id, 3);
    }

    #[test]
    fn test_input() {
        let claims = std::fs::read_to_string("input/claims.txt").unwrap();
        let claims = ClaimsInfo::from(claims.as_str());

        assert_eq!(claims.total_size.x, 999);
        assert_eq!(claims.total_size.y, 1000);
        assert_eq!(claims.count_squares_with_overlapping_claims(), 118322);

        assert_eq!(claims.find_non_overlapping_claim().id, 1178);
    }
}
