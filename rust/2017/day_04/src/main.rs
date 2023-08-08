#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    println!("Advent of Code 2017 - day 04");
}

fn contains_repeated_words(s: &str) -> bool {
    let split = s.split(" ");
    let words: HashSet<&str> = split.clone().collect();

    words.len() != split.count()
}

fn count_valid_passphrases_no_repeated_words(s: &str) -> usize {
    s.lines()
        .map(|line| contains_repeated_words(line))
        .filter(|b| !*b)
        .count()
}

fn contains_anagrams(s: &str) -> bool {
    let sorted_words: Vec<String> = s
        .split(" ")
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            chars.iter().collect::<String>()
        })
        .collect();
    let set: HashSet<&str> = sorted_words.iter().map(|s| s.as_str()).collect();

    sorted_words.len() != set.len()
}

fn count_valid_passphrases_no_anagrams(s: &str) -> usize {
    s.lines()
        .map(|line| !contains_anagrams(line))
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{
        contains_anagrams, contains_repeated_words, count_valid_passphrases_no_anagrams,
        count_valid_passphrases_no_repeated_words,
    };

    #[test]
    fn test_examples() {
        assert_eq!(contains_repeated_words("aa bb cc dd ee"), false);
        assert_eq!(contains_repeated_words("aa bb cc dd aa"), true);
        assert_eq!(contains_repeated_words("aa bb cc dd aaa"), false);

        let input = "\
            aa bb cc dd ee\n\
            aa bb cc dd aa\n\
            aa bb cc dd aaa\
        ";

        assert_eq!(count_valid_passphrases_no_repeated_words(input), 2);

        assert_eq!(contains_anagrams("abcde fghij"), false);
        assert_eq!(contains_anagrams("abcde xyz ecdab"), true);
        assert_eq!(contains_anagrams("a ab abc abd abf abj"), false);
        assert_eq!(contains_anagrams("iiii oiii ooii oooi oooo"), false);
        assert_eq!(contains_anagrams("oiii ioii iioi iiio"), true);

        let input = "\
            abcde fghij\n\
            abcde xyz ecdab\n\
            a ab abc abd abf abj\n\
            iiii oiii ooii oooi oooo\n\
            oiii ioii iioi iiio\
        ";

        assert_eq!(count_valid_passphrases_no_anagrams(&input), 3);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/passphrases.txt").unwrap();

        assert_eq!(count_valid_passphrases_no_repeated_words(&input), 451);

        assert_eq!(count_valid_passphrases_no_anagrams(&input), 223);
    }
}
