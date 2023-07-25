#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 05");
}

fn compute_password(door_id: &str) -> String {
    let mut i: u64 = 0;
    let mut pw = String::new();

    while pw.len() < 8 {
        i += 1;

        let base = format!("{}{}", door_id, i);
        let hash = format!("{:x}", md5::compute(&base));

        if hash.starts_with("00000") {
            pw.push_str(&hash[5..6]);
        }
    }

    return pw;
}

fn compute_password_2(door_id: &str) -> String {
    let mut i: u64 = 0;
    let mut pw = ['_'; 8];
    let mut solved = 0;

    while solved < 8 {
        i += 1;

        let base = format!("{}{}", door_id, i);
        let hash = format!("{:x}", md5::compute(&base));

        if hash.starts_with("00000") {
            let position = (&hash[5..6]).parse::<usize>();
            if let Ok(position) = position {
                if position < 8 && pw[position] == '_' {
                    solved += 1;
                    let ch = *(&hash[6..7].chars().next().unwrap());
                    pw[position] = ch;
                }
            }
        }
    }

    return pw.iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::{compute_password, compute_password_2};

    #[test]
    fn test_example() {
        assert_eq!(compute_password("abc"), "18f47a30");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(compute_password_2("abc"), "05ace8e3");
    }

    #[test]
    fn test_input() {
        assert_eq!(compute_password("uqwqemis"), "1a3099aa");
    }

    #[test]
    fn test_input_2() {
        assert_eq!(compute_password_2("uqwqemis"), "694190cd");
    }
}
