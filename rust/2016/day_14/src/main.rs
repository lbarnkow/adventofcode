#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 14");
}

struct KeyGenerator {
    salt: String,
    key_stretches: usize,
    hashes: Vec<String>,
    keys: Vec<usize>,
}

impl KeyGenerator {
    fn new(salt: &str, key_stretches: usize) -> Self {
        let mut kg = Self {
            salt: salt.to_owned(),
            key_stretches,
            hashes: Vec::new(),
            keys: Vec::new(),
        };

        kg.calculate_hash(0);
        kg
    }

    fn calculate_hash(&mut self, idx: usize) {
        if self.hashes.is_empty() && idx != 0 {
            panic!("No gaps in hash table allowed!");
        }
        if !self.hashes.is_empty() && idx != self.hashes.len() {
            panic!("No gaps in hash table allowed!");
        }

        let salt_and_idx = format!("{}{}", self.salt, idx);
        let mut hash = format!("{:x}", md5::compute(salt_and_idx.as_bytes()));

        for _ in 0..self.key_stretches {
            hash = format!("{:x}", md5::compute(hash));
        }

        self.hashes.push(format!("{hash}"));
    }

    fn get_hash(&mut self, idx: usize) -> &str {
        let max_idx = if !self.hashes.is_empty() {
            self.hashes.len() - 1
        } else {
            0
        };

        if idx > max_idx {
            for i in (max_idx + 1)..=idx {
                self.calculate_hash(i)
            }
        }

        &self.hashes[idx]
    }

    fn has_repeating_char(&mut self, idx: usize, ch: Option<char>, n: usize) -> Option<char> {
        let hash = self.get_hash(idx);

        for i in 0..=(hash.len() - n) {
            let mut chars = hash[i..(i + n)].chars();
            let first = if let Some(ch) = ch {
                ch
            } else {
                chars.next().expect("Should not fail!")
            };
            if chars.fold(true, |matches, c| matches && first == c) {
                return Some(first);
            }
        }

        None
    }

    fn is_key(&mut self, idx: usize) -> bool {
        if let Some(ch) = self.has_repeating_char(idx, None, 3) {
            for n in 1..=1000 {
                if let Some(_) = self.has_repeating_char(idx + n, Some(ch), 5) {
                    return true;
                }
            }
        }

        false
    }

    fn generate_next_key(&mut self) -> (usize, &str) {
        let mut idx = if let Some(k_idx) = self.keys.last() {
            k_idx + 1
        } else {
            0
        };

        loop {
            if self.is_key(idx) {
                self.keys.push(idx);
                return (idx, &self.hashes[idx]);
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::KeyGenerator;

    #[test]
    fn test_example() {
        let mut kg = KeyGenerator::new("abc", 0);
        let mut idx = 0;
        for _ in 0..64 {
            (idx, _) = kg.generate_next_key();
        }
        assert_eq!(idx, 22728);
    }

    #[test]
    fn test_example_part2() {
        let mut kg = KeyGenerator::new("abc", 2016);
        let mut idx = 0;
        for _ in 0..64 {
            (idx, _) = kg.generate_next_key();
        }
        assert_eq!(idx, 22551);
    }

    #[test]
    fn test_input() {
        let mut kg = KeyGenerator::new("ihaygndm", 0);
        let mut idx = 0;
        for _ in 0..64 {
            (idx, _) = kg.generate_next_key();
        }
        assert_eq!(idx, 15035);
    }

    #[test]
    fn test_input_part2() {
        let mut kg = KeyGenerator::new("ihaygndm", 2016);
        let mut idx = 0;
        for _ in 0..64 {
            (idx, _) = kg.generate_next_key();
        }
        assert_eq!(idx, 19968);
    }
}
