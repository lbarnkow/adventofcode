#![allow(dead_code)]
use regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 14");
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u64,
    air_time: u64,
    rest_time: u64,
}

impl Reindeer {
    fn parse(s: &str) -> Vec<Self> {
        let re = Regex::new(
            r"^(\w+) can fly (\d+) km.s for (\d+) seconds, but then must rest for (\d+) seconds.$",
        )
        .unwrap();
        let mut result = Vec::new();

        for line in s.lines() {
            let caps = re.captures(line).unwrap();

            result.push(Self {
                name: caps[1].to_owned(),
                speed: caps[2].parse().unwrap(),
                air_time: caps[3].parse().unwrap(),
                rest_time: caps[4].parse().unwrap(),
            })
        }

        result
    }

    fn calc_travel_distance(&self, seconds: u64) -> u64 {
        let whole_cycles = seconds / (self.air_time + self.rest_time);
        let remainder = seconds - (whole_cycles * (self.air_time + self.rest_time));

        let dist = whole_cycles * self.air_time * self.speed;
        let remainder_dist = if remainder <= self.air_time {
            remainder * self.speed
        } else {
            self.air_time * self.speed
        };

        dist + remainder_dist
    }
}

fn max_travel_distance(reindeer: &str, seconds: u64) -> (Reindeer, u64) {
    let reindeer = Reindeer::parse(reindeer);

    let mut winner: Option<Reindeer> = None;
    let mut max_dist = u64::MIN;

    for r in reindeer {
        let dist = r.calc_travel_distance(seconds);

        if dist > max_dist {
            winner = Some(r);
            max_dist = dist;
        }
    }

    (winner.unwrap(), max_dist)
}

fn max_points_with_new_scoring_system(reindeer: &str, seconds: u64) -> (Reindeer, u64) {
    let mut reindeer = Reindeer::parse(reindeer);
    let mut scores = vec![0; reindeer.len()];

    for s in 1..=seconds {
        let mut leader = Vec::new();
        let mut max_dist = u64::MIN;

        for (idx, reindeer) in reindeer.iter().enumerate() {
            let dist = reindeer.calc_travel_distance(s);
            if dist > max_dist {
                max_dist = dist;
                leader.clear();
                leader.push(idx);
            } else if dist == max_dist {
                leader.push(idx);
            }
        }

        for idx in leader {
            scores[idx] += 1;
        }
    }

    let (idx, score) = scores
        .into_iter()
        .enumerate()
        .max_by(|(_, a_score), (_, b_score)| a_score.cmp(b_score))
        .unwrap();

    (reindeer.remove(idx), score)
}

#[cfg(test)]
mod tests {
    use crate::{max_points_with_new_scoring_system, max_travel_distance, Reindeer};

    #[test]
    fn test_examples() {
        let reindeer = "\
            Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
            Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\
        ";

        let r = Reindeer::parse(reindeer);
        assert_eq!(r.len(), 2);
        assert_eq!(r.get(0).unwrap().name, "Comet");
        assert_eq!(r.get(0).unwrap().speed, 14);
        assert_eq!(r.get(0).unwrap().air_time, 10);
        assert_eq!(r.get(0).unwrap().rest_time, 127);
        assert_eq!(r.get(1).unwrap().name, "Dancer");
        assert_eq!(r.get(1).unwrap().speed, 16);
        assert_eq!(r.get(1).unwrap().air_time, 11);
        assert_eq!(r.get(1).unwrap().rest_time, 162);

        assert_eq!(r.get(0).unwrap().calc_travel_distance(1000), 1120);
        assert_eq!(r.get(1).unwrap().calc_travel_distance(1000), 1056);

        let (winner, dist) = max_travel_distance(reindeer, 1000);
        assert_eq!(winner.name, "Comet");
        assert_eq!(dist, 1120);

        let (winner, score) = max_points_with_new_scoring_system(&reindeer, 1);
        println!("{winner:?}");
        assert_eq!(winner.name, "Dancer");
        assert_eq!(score, 1);

        let (winner, score) = max_points_with_new_scoring_system(&reindeer, 140);
        println!("{winner:?}");
        assert_eq!(winner.name, "Dancer");
        assert_eq!(score, 139);

        let (winner, score) = max_points_with_new_scoring_system(&reindeer, 1000);
        println!("{winner:?}");
        assert_eq!(winner.name, "Dancer");
        assert_eq!(score, 689);
    }

    #[test]
    fn test_input() {
        let reindeer = std::fs::read_to_string("input/reindeer.txt").unwrap();

        let (winner, dist) = max_travel_distance(&reindeer, 2503);
        println!("{winner:?}");
        assert_eq!(dist, 2660);

        let (winner, score) = max_points_with_new_scoring_system(&reindeer, 2503);
        println!("{winner:?}");
        assert_eq!(score, 1256);
    }
}
