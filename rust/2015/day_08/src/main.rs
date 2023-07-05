#![allow(dead_code)]
use regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 8");
}

fn compute_size(strings: &str) -> (usize, usize, usize) {
    let escape_backslash = Regex::new(r"\\\\").unwrap();
    let escape_quote = Regex::new(r#"\\\""#).unwrap();
    let escape_ascii_code = Regex::new(r"\\x[a-f0-9]{2}").unwrap();

    let mut raw_chars = 0;
    let mut chars = 0;

    for line in strings.lines() {
        raw_chars += line.len();

        let line = escape_backslash.replace_all(line, "x");
        let line = escape_quote.replace_all(&line, "x");
        let line = escape_ascii_code.replace_all(&line, "x");
        let line = line.replace('"', "");

        chars += line.len();
    }

    (raw_chars, chars, raw_chars - chars)
}

fn compute_encoded(strings: &str) -> (usize, usize, usize) {
    let mut raw_chars = 0;
    let mut encoded_chars = 0;

    for line in strings.lines() {
        raw_chars += line.len();

        let line = line.replace("\\", "\\\\").replace("\"", "\\\"");

        encoded_chars += line.len() + 2; // add enclosing quotes
    }

    (raw_chars, encoded_chars, encoded_chars - raw_chars)
}

#[cfg(test)]
mod tests {
    use crate::{compute_encoded, compute_size};

    #[test]
    fn test_examples() {
        let strings = concat!(
            r#""""#,
            "\n",
            r#""abc""#,
            "\n",
            r#""aaa\"aaa""#,
            "\n",
            r#""\x27""#
        );
        let (literal_size, mem_size, diff) = compute_size(strings);
        assert_eq!(literal_size, 23);
        assert_eq!(mem_size, 11);
        assert_eq!(diff, 12);

        let (literal_size, encoded_size, diff) = compute_encoded(strings);
        assert_eq!(literal_size, 23);
        assert_eq!(encoded_size, 42);
        assert_eq!(diff, 19);
    }

    #[test]
    fn test_input() {
        let strings = std::fs::read_to_string("input/strings.txt").unwrap();
        let (literal_size, mem_size, diff) = compute_size(&strings);
        assert_eq!(literal_size, 6195);
        assert_eq!(mem_size, 4845);
        assert_eq!(diff, 1350);

        let (literal_size, encoded_size, diff) = compute_encoded(&strings);
        assert_eq!(literal_size, 6195);
        assert_eq!(encoded_size, 8280);
        assert_eq!(diff, 2085);
    }
}
