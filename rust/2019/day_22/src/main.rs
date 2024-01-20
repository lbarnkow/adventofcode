#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Advent of Code 2019 - day 22");

    let shuffles = std::fs::read_to_string("input/shuffles.txt").unwrap();
    let shuffles = parse_shuffles(shuffles.as_str());

    let end_idx = Shuffle::apply_multiple(&shuffles, 2019, 10_007);
    assert_eq!(end_idx, 3324);

    let end_idx = (0..10_006).fold(2019, |acc, _| {
        Shuffle::apply_multiple(&shuffles, acc, 10_007)
    });
    assert_eq!(end_idx, 2019);

    // let num_cards = 17_574_135_437_386;

    let shuffles = GeneralizedShuffle::from_shuffles(&shuffles, 10_007);
    let end_idx = shuffles.apply(2019);
    assert_eq!(end_idx, 3324);

    let num_cards = 119315717514047_i128;
    let rounds = 101741582076661_i128;
    let mut rounds_to_loop = (num_cards - 1) - rounds;

    let mut i = 0;
    while rounds_to_loop > 1 {
        println!("{rounds_to_loop}");
        rounds_to_loop /= 2;
        i += 1;
    }
    println!("i = {i}");
}

struct TryFromError {
    msg: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

impl TryFrom<&str> for Shuffle {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "deal into new stack" {
            Ok(Self::DealIntoNewStack)
        } else if let Some(stripped) = value.strip_prefix("cut ") {
            let n = stripped.parse().unwrap();
            Ok(Self::Cut(n))
        } else if let Some(stripped) = value.strip_prefix("deal with increment ") {
            let n = stripped.parse().unwrap();
            Ok(Self::DealWithIncrement(n))
        } else {
            Err(Self::Error {
                msg: format!("Unrecognized shuffle technique: '{value}'!"),
            })
        }
    }
}

impl Shuffle {
    const fn apply(&self, card: i128, num_cards: i128) -> i128 {
        match self {
            Self::DealIntoNewStack => Self::deal_into_new_stack(card, num_cards),
            Self::Cut(n) => Self::cut(card, num_cards, *n),
            Self::DealWithIncrement(n) => Self::deal_with_increment(card, num_cards, *n),
        }
    }

    fn apply_multiple(shuffles: &[Self], card: i128, num_cards: i128) -> i128 {
        shuffles
            .iter()
            .fold(card, |acc, shuffle| shuffle.apply(acc, num_cards))
    }

    const fn deal_into_new_stack(card: i128, num_cards: i128) -> i128 {
        /*
        0 1 2 3 4 5 6 7 8 9
        9 8 7 6 5 4 3 2 1 0
        */
        (-card + num_cards - 1) % num_cards
    }

    const fn cut(card: i128, num_cards: i128, n: i128) -> i128 {
        /*
        0 1 2 3 4 5 6 7 8 9 ; n=4
        0 1 2 3
                4 5 6 7 8 9
        4 5 6 7 8 9 0 1 2 3

        0 1 2 3 4 5 6 7 8 9 ; n=-4
                    6 7 8 9
        0 1 2 3 4 5
        6 7 8 9 0 1 2 3 4 5
         */

        (card + num_cards - n) % num_cards
    }

    const fn deal_with_increment(card: i128, num_cards: i128, n: i128) -> i128 {
        /*
        0 1 2 3 4 5 6 7 8 9 ; n=3
        0 7 4 1 8 5 2 9 6 3
        */
        (n * card) % num_cards
    }
}

#[derive(Debug, Clone, Copy)]
struct GeneralizedShuffle {
    a: i128,
    b: i128,
    num_cards: i128,
}

impl GeneralizedShuffle {
    const fn new(a: i128, b: i128, num_cards: i128) -> Self {
        Self { a, b, num_cards }
    }

    const fn apply(&self, card: i128) -> i128 {
        let result = (self.a * card + self.b) % self.num_cards;
        // a and/or b may possibly be large negative numbers. thus, result
        // may end up between -self.num_cards and 0.
        (result + self.num_cards) % self.num_cards
    }

    fn apply_multiple(shuffles: &[Self], card: i128) -> i128 {
        shuffles.iter().fold(card, |acc, s| s.apply(acc))
    }

    const fn combine_with(&self, other: &Self) -> Self {
        /*
        s1(card) = (a'*card + b') % N
        s2(card) = (a''*card + b'') % N

        x = s2(s1(card))
        x = (a''*s1(card) + b'') % N
        x = (a'' * ((a' * card + b') % N) + b'') % N
        x = (a'' * (a' * card + b') + b'') % N
        x = (a'' * a' * card + a'' * b' + b'') % N
        */
        Self {
            a: (other.a * self.a) % self.num_cards,
            b: (other.a * self.b + other.b) % self.num_cards,
            num_cards: self.num_cards,
        }
    }

    fn apply_repeatedly(&self, card: i128, mut times: i128) -> i128 {
        // The deck ends up sorted again after applying the shuffles N-1 times (N being the number cards in the deck).
        // https://www.reddit.com/r/adventofcode/comments/ee56wh/comment/fbr0vjb/

        let shuffle_loop = self.num_cards - 1;
        times %= shuffle_loop;

        let mut remaining_rounds_until_sorted = shuffle_loop - times;
        let mut shuffles = *self;
        let mut rounds = 1;
        let mut map = HashMap::new();
        while rounds < remaining_rounds_until_sorted {
            map.insert(rounds, shuffles);
            rounds *= 2;
            shuffles = shuffles.combine_with(&shuffles);
        }

        shuffles = Self::new(1, 0, self.num_cards); // neutral shuffle
        let mut map: Vec<(i128, Self)> = map.into_iter().collect();
        map.sort_by(|(a, _), (b, _)| b.cmp(a));
        for (key, value) in map {
            if key <= remaining_rounds_until_sorted {
                shuffles = shuffles.combine_with(&value);
                remaining_rounds_until_sorted -= key;
            }
        }
        assert_eq!(remaining_rounds_until_sorted, 0);

        shuffles.apply(card)
    }

    const fn from_shuffle(value: &Shuffle, num_cards: i128) -> Self {
        match value {
            Shuffle::DealIntoNewStack => Self {
                a: -1,
                b: num_cards - 1,
                num_cards,
            },
            Shuffle::Cut(n) => Self {
                a: 1,
                b: num_cards - *n,
                num_cards,
            },
            Shuffle::DealWithIncrement(n) => Self {
                a: *n,
                b: 0,
                num_cards,
            },
        }
    }

    fn from_shuffles(shuffles: &[Shuffle], num_cards: i128) -> Self {
        let mut shuffles: VecDeque<Self> = shuffles
            .iter()
            .map(|s| Self::from_shuffle(s, num_cards))
            .collect();

        while shuffles.len() > 1 {
            let s1 = shuffles.pop_front().unwrap();
            let s2 = shuffles.pop_front().unwrap();
            shuffles.push_front(s1.combine_with(&s2));
        }

        shuffles.pop_front().unwrap()
    }
}

fn parse_shuffles(s: &str) -> Vec<Shuffle> {
    s.lines()
        .map(Shuffle::try_from)
        .map(|e| match e {
            Ok(s) => s,
            Err(e) => panic!("{}", e.msg),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_shuffles, GeneralizedShuffle, Shuffle};

    fn parse_result(s: &str) -> Vec<i128> {
        assert!(s.starts_with("Result: "));
        (&s[8..])
            .split(' ')
            .map(|split| split.parse().unwrap())
            .collect()
    }

    fn perform_test(shuffles: &str, result: &str) {
        let shuffles = parse_shuffles(shuffles);
        let result = parse_result(result);

        for result_idx in 0..result.len() {
            let start_idx = result[result_idx];
            let end_idx = Shuffle::apply_multiple(&shuffles, start_idx, result.len() as i128);
            assert_eq!(end_idx, result_idx as i128);
        }

        let shuffle = GeneralizedShuffle::from_shuffles(&shuffles, result.len() as i128);
        for result_idx in 0..result.len() {
            let start_idx = result[result_idx];
            let end_idx = shuffle.apply(start_idx);
            assert_eq!(end_idx, result_idx as i128);
        }
    }

    #[test]
    fn test_example_1() {
        let shuffles = "\
            deal with increment 7\n\
            deal into new stack\n\
            deal into new stack\
            ";
        let result = "Result: 0 3 6 9 2 5 8 1 4 7";
        perform_test(shuffles, result);
    }

    #[test]
    fn test_example_2() {
        let shuffles = "\
            cut 6\n\
            deal with increment 7\n\
            deal into new stack\
        ";
        let result = "Result: 3 0 7 4 1 8 5 2 9 6";
        perform_test(shuffles, result);
    }

    #[test]
    fn test_example_3() {
        let shuffles = "\
            deal with increment 7\n\
            deal with increment 9\n\
            cut -2\
        ";
        let result = "Result: 6 3 0 7 4 1 8 5 2 9";
        perform_test(shuffles, result);
    }

    #[test]
    fn test_example_4() {
        let shuffles = "\
            deal into new stack\n\
            cut -2\n\
            deal with increment 7\n\
            cut 8\n\
            cut -4\n\
            deal with increment 7\n\
            cut 3\n\
            deal with increment 9\n\
            deal with increment 3\n\
            cut -1\
        ";
        let result = "Result: 9 2 5 8 1 4 7 0 3 6";
        perform_test(shuffles, result);
    }

    #[test]
    fn test_input() {
        let shuffles = std::fs::read_to_string("input/shuffles.txt").unwrap();
        let shuffles = parse_shuffles(shuffles.as_str());

        let end_idx = Shuffle::apply_multiple(&shuffles, 2019, 10_007);
        assert_eq!(end_idx, 3324);

        let shuffles = GeneralizedShuffle::from_shuffles(&shuffles, 10_007);
        let end_idx = shuffles.apply(2019);
        assert_eq!(end_idx, 3324);
    }

    #[test]
    fn test_input_part2() {
        let shuffles = std::fs::read_to_string("input/shuffles.txt").unwrap();
        let shuffles = parse_shuffles(shuffles.as_str());

        let gen_shuffles = GeneralizedShuffle::from_shuffles(&shuffles, 10_007);
        let idx_before = gen_shuffles.apply_repeatedly(3324, 1);
        assert_eq!(idx_before, 2019);

        let gen_shuffles = GeneralizedShuffle::from_shuffles(&shuffles, 119_315_717_514_047);
        let idx_before = gen_shuffles.apply_repeatedly(2020, 101_741_582_076_661);
        assert_eq!(idx_before, 74132511136410);
    }
}
