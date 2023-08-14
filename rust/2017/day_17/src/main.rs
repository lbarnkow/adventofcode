#![allow(dead_code)]

use std::collections::VecDeque;

fn main() {
    println!("Advent of Code 2017 - day 17");
}

#[derive(Debug)]
struct Spinlock {
    step_len: usize,
    buffer: VecDeque<usize>,
}

impl Spinlock {
    fn new(step_len: usize) -> Self {
        Self {
            step_len,
            buffer: vec![0].into(),
        }
    }

    fn spin(&mut self) {
        self.buffer.rotate_left(self.step_len % self.buffer.len());
        self.buffer.rotate_left(1);
        self.buffer.push_front(self.buffer.len());
    }
}

struct FakeSpinlock {
    step_len: usize,
    len: usize,
    current_idx: usize,
    value_at_idx_1: Option<usize>,
}

impl FakeSpinlock {
    fn new(step_len: usize) -> Self {
        Self {
            step_len,
            len: 1,
            current_idx: 0,
            value_at_idx_1: None,
        }
    }

    fn spin(&mut self) {
        self.current_idx = (self.current_idx + self.step_len) % self.len;
        self.current_idx += 1;
        if self.current_idx == 1 {
            self.value_at_idx_1 = Some(self.len);
        }
        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{FakeSpinlock, Spinlock};

    #[test]
    fn test_examples() {
        let mut lock = Spinlock::new(3);

        lock.spin();
        assert_eq!(lock.buffer[0], 1);
        assert_eq!(lock.buffer[1], 0);

        lock.spin();
        assert_eq!(lock.buffer[0], 2);
        assert_eq!(lock.buffer[1], 1);

        lock.spin();
        assert_eq!(lock.buffer[0], 3);
        assert_eq!(lock.buffer[1], 1);

        lock.spin();
        assert_eq!(lock.buffer[0], 4);
        assert_eq!(lock.buffer[1], 3);

        lock.spin();
        assert_eq!(lock.buffer[0], 5);
        assert_eq!(lock.buffer[1], 2);

        lock.spin();
        assert_eq!(lock.buffer[0], 6);
        assert_eq!(lock.buffer[1], 1);

        lock.spin();
        assert_eq!(lock.buffer[0], 7);
        assert_eq!(lock.buffer[1], 2);

        lock.spin();
        assert_eq!(lock.buffer[0], 8);
        assert_eq!(lock.buffer[1], 6);

        lock.spin();
        assert_eq!(lock.buffer[0], 9);
        assert_eq!(lock.buffer[1], 5);

        let mut lock = Spinlock::new(3);

        for _ in 0..2017 {
            lock.spin();
        }

        assert_eq!(lock.buffer[0], 2017);
        assert_eq!(lock.buffer[1], 638);
    }

    #[test]
    fn test_examples_part2() {
        let mut lock = FakeSpinlock::new(3);
        for _ in 0..50_000_000 {
            lock.spin();
        }

        assert_eq!(lock.len, 50_000_001);
        assert_eq!(lock.value_at_idx_1, Some(1222153));
    }

    #[test]
    fn test_input() {
        let mut lock = Spinlock::new(371);

        for _ in 0..2017 {
            lock.spin();
        }

        assert_eq!(lock.buffer[0], 2017);
        assert_eq!(lock.buffer[1], 1311);
    }

    #[test]
    fn test_input_part2() {
        let mut lock = FakeSpinlock::new(371);
        for _ in 0..50_000_000 {
            lock.spin();
        }

        assert_eq!(lock.len, 50_000_001);
        assert_eq!(lock.value_at_idx_1, Some(39170601));
    }
}
