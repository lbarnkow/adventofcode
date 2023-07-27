#![allow(dead_code)]

use std::str::CharIndices;

fn main() {
    println!("Advent of Code 2016 - day 09");
}

fn read_marker(iter: &mut CharIndices<'_>) -> (usize, usize, usize) {
    let mut marker = String::new();
    let mut consumed = 0;

    while let Some((_, c)) = iter.next() {
        consumed += 1;
        match c {
            ')' => break,
            _ => marker.push(c),
        }
    }

    let mut marker = marker.split('x');

    (
        consumed,
        marker.next().unwrap().parse::<usize>().unwrap(),
        marker.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn decompressed_len(s: &str, recusive: bool) -> usize {
    let mut count = 0;
    let mut iter = s.char_indices();

    while let Some((idx, c)) = iter.next() {
        if c != '(' {
            count += 1;
            continue;
        }

        let (consumed, len, reps) = read_marker(&mut iter);
        let idx = idx + consumed + 1;
        if recusive {
            count += reps * decompressed_len(&s[idx..idx + len], recusive);
        } else {
            count += reps * len;
        }

        for _ in 0..len {
            iter.next().unwrap();
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::decompressed_len;

    #[test]
    fn test_example() {
        assert_eq!(decompressed_len("ADVENT", false), 6);
        assert_eq!(decompressed_len("A(1x5)BC", false), 7);
        assert_eq!(decompressed_len("(3x3)XYZ", false), 9);
        assert_eq!(decompressed_len("A(2x2)BCD(2x2)EFG", false), 11);
        assert_eq!(decompressed_len("(6x1)(1x3)A", false), 6);
        assert_eq!(decompressed_len("X(8x2)(3x3)ABCY", false), 18);

        assert_eq!(decompressed_len("(3x3)XYZ", true), 9);
        assert_eq!(decompressed_len("X(8x2)(3x3)ABCY", true), 20);
        assert_eq!(
            decompressed_len("(27x12)(20x12)(13x14)(7x10)(1x12)A", true),
            241920
        );
        assert_eq!(
            decompressed_len(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                true
            ),
            445
        );
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/file.txt").unwrap();
        assert_eq!(decompressed_len(&input, false), 115_118);
        assert_eq!(decompressed_len(&input, true), 11_107_527_530);
    }
}
