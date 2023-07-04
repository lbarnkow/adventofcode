#![allow(dead_code)]
use std::{collections::HashSet, fmt::Debug};

fn main() {
    println!("Advent of Code 2015 - day 4");
}

fn mine(key: &str, prefix: &str) -> usize {
    let mut i = 0;

    loop {
        i += 1;

        let s = format!("{}{}", key, i);
        let digest = md5::compute(s.as_bytes());
        let hash = format!("{:x}", digest);

        if hash.starts_with(prefix) {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mine;

    #[test]
    fn test_examples() {
        assert_eq!(mine("abcdef", "00000"), 609043);
        assert_eq!(mine("pqrstuv", "00000"), 1048970);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(mine("iwrupvqb", "00000"), 346386);
        assert_eq!(mine("iwrupvqb", "000000"), 9958218);
    }
}
