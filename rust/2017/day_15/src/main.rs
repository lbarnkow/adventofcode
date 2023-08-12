#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 15");
}

const QUOTIENT: u64 = 2147483647;

trait Generator {
    fn next(&mut self) -> u64;
}

struct SimpleGenerator {
    prev: u64,
    factor: u64,
    quotient: u64,
}

impl SimpleGenerator {
    fn new(seed: u64, factor: u64, quotient: u64) -> Self {
        Self {
            prev: seed,
            factor,
            quotient,
        }
    }
}

impl Generator for SimpleGenerator {
    fn next(&mut self) -> u64 {
        self.prev = (self.prev * self.factor) % self.quotient;
        self.prev
    }
}

struct PickyGenerator {
    gen: SimpleGenerator,
    crit: u64,
}

impl PickyGenerator {
    fn new(seed: u64, factor: u64, quotient: u64, crit: u64) -> Self {
        Self {
            gen: SimpleGenerator::new(seed, factor, quotient),
            crit,
        }
    }
}

impl Generator for PickyGenerator {
    fn next(&mut self) -> u64 {
        loop {
            let next = self.gen.next();
            if next % self.crit == 0 {
                return next;
            }
        }
    }
}

fn judge<Gen>(gen_a: &mut Gen, gen_b: &mut Gen, n: usize) -> usize
where
    Gen: Generator,
{
    let mut count = 0;

    for _ in 0..n {
        let a = gen_a.next() & 0xffff;
        let b = gen_b.next() & 0xffff;

        if a == b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{judge, PickyGenerator, SimpleGenerator, QUOTIENT};

    #[test]
    fn test_examples() {
        let mut gen_a = SimpleGenerator::new(65, 16807, QUOTIENT);
        let mut gen_b = SimpleGenerator::new(8921, 48271, QUOTIENT);

        let count = judge(&mut gen_a, &mut gen_b, 40_000_000);
        assert_eq!(count, 588);
    }

    #[test]
    fn test_examples_part2() {
        let mut gen_a = PickyGenerator::new(65, 16807, QUOTIENT, 4);
        let mut gen_b = PickyGenerator::new(8921, 48271, QUOTIENT, 8);

        let count = judge(&mut gen_a, &mut gen_b, 5_000_000);
        assert_eq!(count, 309);
    }

    #[test]
    fn test_input() {
        let mut gen_a = SimpleGenerator::new(634, 16807, QUOTIENT);
        let mut gen_b = SimpleGenerator::new(301, 48271, QUOTIENT);

        let count = judge(&mut gen_a, &mut gen_b, 40_000_000);
        assert_eq!(count, 573);
    }

    #[test]
    fn test_input_part2() {
        let mut gen_a = PickyGenerator::new(634, 16807, QUOTIENT, 4);
        let mut gen_b = PickyGenerator::new(301, 48271, QUOTIENT, 8);

        let count = judge(&mut gen_a, &mut gen_b, 5_000_000);
        assert_eq!(count, 294);
    }
}
