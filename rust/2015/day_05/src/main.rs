#![allow(dead_code)]
use fancy_regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 5");
}

fn is_nice(string: &str) -> bool {
    let has_at_least_three_vowels = Regex::new(r"([aeiou].*){3}").unwrap();
    let has_at_least_one_double_letter = Regex::new(r"(\w)\1").unwrap();
    let has_forbidden_strings = Regex::new(r"ab|cd|pq|xy").unwrap();

    !has_forbidden_strings.is_match(string).unwrap() &&
    has_at_least_three_vowels.is_match(string).unwrap() &&
    has_at_least_one_double_letter.is_match(string).unwrap()
}

fn is_nice_new(string: &str) -> bool {
    let has_a_recurring_two_letter_pair = Regex::new(r"(\w\w).*\1").unwrap();
    let has_recurring_character_with_one_letter_in_between = Regex::new(r"(\w)\w\1").unwrap();

    has_a_recurring_two_letter_pair.is_match(string).unwrap() &&
    has_recurring_character_with_one_letter_in_between.is_match(string).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{is_nice, is_nice_new};

    #[test]
    fn test_examples() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_input() {
        let lines = std::fs::read_to_string("input/strings.txt").unwrap();

        let mut nice_count = 0;

        for line in lines.lines() {
            if is_nice(line) {
                nice_count += 1;
            }
        }

        assert_eq!(nice_count, 236);
    }

    #[test]
    fn test_input_new() {
        let lines = std::fs::read_to_string("input/strings.txt").unwrap();

        let mut nice_count = 0;

        for line in lines.lines() {
            if is_nice_new(line) {
                nice_count += 1;
            }
        }

        assert_eq!(nice_count, 51);
    }
}
