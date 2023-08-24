#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2018 - day 02");
}

fn has_repeated_chars(id: &str, n: usize) -> bool {
    let mut histogram = HashMap::new();

    for c in id.chars() {
        if let Some(count) = histogram.get_mut(&c) {
            *count += 1;
        } else {
            histogram.insert(c, 1);
        }
    }
    histogram.iter().filter(|(_, v)| **v == n).count() > 0
}

fn checksum(boxes: &str) -> usize {
    let n2 = boxes.lines().filter(|id| has_repeated_chars(id, 2)).count();
    let n3 = boxes.lines().filter(|id| has_repeated_chars(id, 3)).count();

    n2 * n3
}

fn are_ids_similar(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|(a, b)| *a != *b).count() == 1
}

fn base_id(a: &str, b: &str) -> Option<String> {
    if !are_ids_similar(a, b) {
        return None;
    }

    Some(
        a.chars()
            .zip(b.chars())
            .filter(|(a, b)| *a == *b)
            .map(|(a, _)| a)
            .collect(),
    )
}

fn find_similar_pair(boxes: &str) -> String {
    let boxes: Vec<&str> = boxes.lines().collect();

    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            if let Some(base_id) = base_id(boxes[i], boxes[j]) {
                return base_id;
            }
        }
    }
    panic!("No similiar box ids found!");
}

#[cfg(test)]
mod tests {
    use crate::{checksum, find_similar_pair};

    #[test]
    fn test_examples() {
        let boxes = "\
            abcdef\n\
            bababc\n\
            abbcde\n\
            abcccd\n\
            aabcdd\n\
            abcdee\n\
            ababab\
        ";

        assert_eq!(checksum(boxes), 12);
    }

    #[test]
    fn test_examples_part2() {
        let boxes = "\
            abcde\n\
            fghij\n\
            klmno\n\
            pqrst\n\
            fguij\n\
            axcye\n\
            wvxyz\
        ";

        assert_eq!(find_similar_pair(boxes), "fgij");
    }

    #[test]
    fn test_input() {
        let boxes = std::fs::read_to_string("input/boxes.txt").unwrap();

        assert_eq!(checksum(&boxes), 4940);
        assert_eq!(find_similar_pair(&boxes), "wrziyfdmlumeqvaatbiosngkc");
    }
}
