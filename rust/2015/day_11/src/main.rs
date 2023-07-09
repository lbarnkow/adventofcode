#![allow(dead_code)]

static ILLEGAL_CHARS: [char; 3] = ['i', 'o', 'l'];

fn increment_char(c: char) -> char {
    unsafe {
        if c == 'z' {
            'a'
        } else {
            char::from_u32_unchecked((c as u32) + 1)
        }
    }
}

fn increment_str(s: &mut Vec<char>) {
    let mut idx = s.len() - 1;

    loop {
        s[idx] = increment_char(s[idx]);

        if s[idx] != 'a' {
            break;
        }

        if idx == 0 {
            s.insert(0, 'a');
            break;
        }

        idx -= 1;
    }
}

fn has_increasing_straight_of_chars(s: &Vec<char>, len: usize) -> bool {
    if s.len() < len {
        panic!("Haystack string is shorter than required straight of chars!");
    }

    let mut prev = '@';
    let mut count = 1;

    for c in s {
        if prev < *c && increment_char(prev) == *c {
            count += 1;
        } else {
            count = 1;
        }
        prev = *c;

        if count == len {
            return true;
        }
    }

    false
}

fn has_no_ambiguous_chars(s: &Vec<char>) -> bool {
    for c in s {
        if ILLEGAL_CHARS.contains(c) {
            return false;
        }
    }

    true
}

fn has_two_non_overlapping_char_pairs(s: &Vec<char>) -> bool {
    let mut i = 0;

    let mut buffer = '@';
    let mut found_pair = false;

    while i < s.len() {
        if buffer == s[i] {
            found_pair = true;
            break;
        } else {
            buffer = s[i];
        }
        i += 1;
    }

    if !found_pair {
        return false;
    }

    let first_pair = buffer;
    buffer = '@';
    i += 1;

    while i < s.len() {
        if buffer == s[i] && first_pair != s[i] {
            return true;
        } else {
            buffer = s[i];
        }
        i += 1;
    }

    false
}

fn is_legal_password(s: &Vec<char>) -> bool {
    has_no_ambiguous_chars(s)
        && has_increasing_straight_of_chars(s, 3)
        && has_two_non_overlapping_char_pairs(s)
}

fn compute_next_legal_password(s: &str) -> String {
    let mut s: Vec<char> = str_to_vec(s);

    loop {
        increment_str(&mut s);
        if is_legal_password(&s) {
            return s.into_iter().collect();
        }
    }
}

fn str_to_vec(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn main() {
    println!("Advent of Code 2015 - day 11");
}

#[cfg(test)]
mod tests {
    use crate::{
        compute_next_legal_password, has_increasing_straight_of_chars, has_no_ambiguous_chars,
        has_two_non_overlapping_char_pairs, is_legal_password, str_to_vec,
    };

    #[test]
    fn test_examples() {
        assert_eq!(
            has_increasing_straight_of_chars(&str_to_vec("aaabcdecc"), 5),
            true
        );

        let pw = "hijklmmn";
        assert_eq!(has_increasing_straight_of_chars(&str_to_vec(pw), 3), true);
        assert_eq!(has_no_ambiguous_chars(&str_to_vec(pw)), false);
        assert_eq!(has_two_non_overlapping_char_pairs(&str_to_vec(pw)), false);
        assert_eq!(is_legal_password(&str_to_vec(pw)), false);

        let pw = "abbceffg";
        assert_eq!(has_increasing_straight_of_chars(&str_to_vec(pw), 3), false);
        assert_eq!(has_no_ambiguous_chars(&str_to_vec(pw)), true);
        assert_eq!(has_two_non_overlapping_char_pairs(&str_to_vec(pw)), true);
        assert_eq!(is_legal_password(&str_to_vec(pw)), false);

        let pw = "abbcegjk";
        assert_eq!(has_increasing_straight_of_chars(&str_to_vec(pw), 3), false);
        assert_eq!(has_no_ambiguous_chars(&str_to_vec(pw)), true);
        assert_eq!(has_two_non_overlapping_char_pairs(&str_to_vec(pw)), false);
        assert_eq!(is_legal_password(&str_to_vec(pw)), false);

        assert_eq!(&compute_next_legal_password("abcdefgh"), "abcdffaa");
        assert_eq!(&compute_next_legal_password("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn test_input() {
        let s = compute_next_legal_password("cqjxjnds");
        assert_eq!(&s, "cqjxxyzz");
        let s = compute_next_legal_password(&s);
        assert_eq!(&s, "cqkaabcc");
    }
}
