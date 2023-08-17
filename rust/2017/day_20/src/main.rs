#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2017 - day 20");
}

lazy_static! {
    static ref RE_PARTICLE: Regex = Regex::new(
        r"^p=<(-?\d+),(-?\d+),(-?\d+)>,\s+v=<(-?\d+),(-?\d+),(-?\d+)>,\s+a=<(-?\d+),(-?\d+),(-?\d+)>$"
    )
    .unwrap();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<usize> for Vector {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs: isize = rhs.try_into().unwrap();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<usize> for Vector {
    fn mul_assign(&mut self, rhs: usize) {
        let rhs: isize = rhs.try_into().unwrap();
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<usize> for Vector {
    type Output = Self;

    fn div(self, rhs: usize) -> Self::Output {
        let rhs: isize = rhs.try_into().unwrap();
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::DivAssign<usize> for Vector {
    fn div_assign(&mut self, rhs: usize) {
        let rhs: isize = rhs.try_into().unwrap();
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vector {
    fn dist(&self, pos: Self) -> isize {
        (self.x - pos.x).abs() + (self.y - pos.y).abs() + (self.z - pos.z).abs()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl From<&str> for Particle {
    fn from(value: &str) -> Self {
        let caps = RE_PARTICLE.captures(value).unwrap();

        let pos = Vector::new(
            caps[1].parse().unwrap(),
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
        );
        let vel = Vector::new(
            caps[4].parse().unwrap(),
            caps[5].parse().unwrap(),
            caps[6].parse().unwrap(),
        );
        let acc = Vector::new(
            caps[7].parse().unwrap(),
            caps[8].parse().unwrap(),
            caps[9].parse().unwrap(),
        );

        Self { pos, vel, acc }
    }
}

impl Particle {
    fn new(pos: Vector, vel: Vector, acc: Vector) -> Self {
        Self { pos, vel, acc }
    }

    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    fn dist(&self, pos: Vector) -> isize {
        self.pos.dist(pos)
    }

    fn update_multi(&mut self, n: usize) {
        // see https://en.wikipedia.org/wiki/Acceleration#Uniform_acceleration

        // v(t) = v0 + a*t
        let v_t = self.vel + (self.acc * n);
        // s(t) = s0 + 0.5 * (v0 + v(t)) * t
        let s_t = self.pos + ((self.vel + v_t) * (n)) / 2;

        self.vel = v_t;
        self.pos = s_t;
    }
}

#[derive(Debug)]
struct ParticleSystem {
    particles: Vec<Particle>,
}

impl From<&str> for ParticleSystem {
    fn from(value: &str) -> Self {
        Self {
            particles: value.lines().map(|line| line.into()).collect(),
        }
    }
}

impl ParticleSystem {
    fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.update();
        }
    }

    fn update_multi(&mut self, n: usize) {
        for p in self.particles.iter_mut() {
            p.update_multi(n);
        }
    }

    fn update_with_collisions(&mut self, n: usize) {
        let mut positions: HashMap<Vector, usize> = HashMap::with_capacity(self.particles.len());

        for _ in 0..n {
            for p in self.particles.iter_mut() {
                p.update();
                if let Some(count) = positions.get_mut(&p.pos) {
                    *count += 1;
                } else {
                    positions.insert(p.pos, 1);
                }
            }
            self.particles
                .retain(|p| *positions.get(&p.pos).unwrap() == 1);
            positions.clear();
        }
    }

    fn closest_particle_to(&self, pos: Vector) -> usize {
        self.particles
            .iter()
            .enumerate()
            .fold((0, isize::MAX), |(closest_idx, closest_dist), (idx, p)| {
                let dist = p.dist(pos);
                if dist < closest_dist {
                    (idx, dist)
                } else {
                    (closest_idx, closest_dist)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Particle, ParticleSystem, Vector};

    #[test]
    fn test_examples() {
        let listing = "\
            p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n\
            p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\
        ";
        let mut particles = ParticleSystem::from(listing);
        let origin = Vector::default();

        assert_eq!(
            particles.particles[0],
            Particle::new(
                Vector::new(3, 0, 0),
                Vector::new(2, 0, 0),
                Vector::new(-1, 0, 0)
            )
        );
        assert_eq!(
            particles.particles[1],
            Particle::new(
                Vector::new(4, 0, 0),
                Vector::new(0, 0, 0),
                Vector::new(-2, 0, 0)
            )
        );
        assert_eq!(particles.closest_particle_to(origin), 0);

        particles.update();
        assert_eq!(
            particles.particles[0],
            Particle::from("p=<4,0,0>, v=<1,0,0>, a=<-1,0,0>")
        );
        assert_eq!(
            particles.particles[1],
            Particle::from("p=<2,0,0>, v=<-2,0,0>, a=<-2,0,0>")
        );
        assert_eq!(particles.closest_particle_to(origin), 1);

        particles.update();
        assert_eq!(
            particles.particles[0],
            Particle::from("p=<4,0,0>, v=<0,0,0>, a=<-1,0,0>")
        );
        assert_eq!(
            particles.particles[1],
            Particle::from("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>")
        );
        assert_eq!(particles.closest_particle_to(origin), 1);

        particles.update();
        assert_eq!(
            particles.particles[0],
            Particle::from("p=<3,0,0>, v=<-1,0,0>, a=<-1,0,0>")
        );
        assert_eq!(
            particles.particles[1],
            Particle::from("p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>")
        );
        assert_eq!(particles.closest_particle_to(origin), 0);

        let mut particles = ParticleSystem::from(listing);
        particles.update_multi(3);
        assert_eq!(particles.closest_particle_to(origin), 0);
    }

    #[test]
    fn test_examples_part2() {
        let listing = "\
            p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n\
            p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n\
            p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n\
            p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\
        ";
        let mut particles = ParticleSystem::from(listing);

        particles.update_with_collisions(1_000);
        assert_eq!(particles.particles.len(), 1);
    }

    #[test]
    fn test_input() {
        let listing = std::fs::read_to_string("input/particles.txt").unwrap();
        let mut particles = ParticleSystem::from(listing.as_str());
        let origin = Vector::default();

        particles.update_multi(1_000);
        assert_eq!(particles.closest_particle_to(origin), 376);
    }

    #[test]
    fn test_input_part2() {
        let listing = std::fs::read_to_string("input/particles.txt").unwrap();
        let mut particles = ParticleSystem::from(listing.as_str());

        particles.update_with_collisions(1_000);
        assert_eq!(particles.particles.len(), 574);
    }
}
