#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 16");
}

fn dragon_curve(input: &str, min_len: usize) -> String {
    let mut data = String::from(input);

    while data.len() < min_len {
        let mut b = String::with_capacity(data.len());
        b.push('0');
        for c in data.chars().rev() {
            b.push(match c {
                '0' => '1',
                '1' => '0',
                _ => panic!("Illegal character in input!"),
            });
        }
        data.reserve(b.len());
        data.push_str(&b);
    }

    data
}

fn checksum(input: &str) -> String {
    if input.len() % 2 != 0 {
        panic!("Cannot checksum odd number of characters!");
    }

    let mut input = String::from(input);
    let mut checksum = String::with_capacity(input.len() / 2);
    loop {
        let mut iter = input.chars();
        while let Some(c1) = iter.next() {
            let c2 = iter.next().expect("Should not be empty!");
            checksum.push(if c1 == c2 { '1' } else { '0' });
        }
        if checksum.len() % 2 == 1 {
            break;
        }
        input = checksum;
        checksum = String::with_capacity(input.len() / 2);
    }

    checksum
}

fn checksum_for_disc(input: &str, disc_size: usize) -> String {
    let data = dragon_curve(input, disc_size);
    checksum(&data[0..disc_size])
}

#[cfg(test)]
mod tests {
    use crate::{checksum, checksum_for_disc, dragon_curve};

    #[test]
    fn test_dragon_curve() {
        assert_eq!(dragon_curve("1", 3), "100");
        assert_eq!(dragon_curve("0", 3), "001");
        assert_eq!(dragon_curve("11111", 11), "11111000000");
        assert_eq!(
            dragon_curve("111100001010", 25),
            "1111000010100101011110000"
        );
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("110010110100"), "100");
        assert_eq!(checksum("10000011110010000111"), "01100");
    }

    #[test]
    fn test_example() {
        assert_eq!(checksum_for_disc("10000", 20), "01100");
    }

    #[test]
    fn test_input() {
        assert_eq!(
            checksum_for_disc("10001001100000001", 272),
            "10101001010100001"
        );

        assert_eq!(
            checksum_for_disc("10001001100000001", 35651584),
            "10100001110101001"
        );
    }
}
