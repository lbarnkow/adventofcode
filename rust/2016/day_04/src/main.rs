#![allow(dead_code)]

use std::collections::HashMap;

use regex::Regex;

fn main() {
    println!("Advent of Code 2016 - day 04");
}

const REGEX: &str = r"^([\w\-]+\w)-(\d+)\[(\w{5})\]$";

fn is_real_room(name: &str, checksum: &str) -> bool {
    assert_eq!(checksum.len(), 5);

    let hist =
        name.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .fold(HashMap::new(), |mut acc, c| {
                if let Some(count) = acc.get(&c) {
                    acc.insert(c, *count + 1);
                } else {
                    acc.insert(c, 1);
                };
                acc
            });

    let mut hist: Vec<(char, i32)> = hist.iter().map(|(c, i)| (*c, *i)).collect();
    hist.sort_by(|(a_char, a_count), (b_char, b_count)| {
        let cmp = b_count.cmp(a_count);
        if cmp.is_eq() {
            a_char.cmp(b_char)
        } else {
            cmp
        }
    });

    for (idx, c) in checksum.chars().enumerate() {
        let hc = (&hist[idx]).0;
        if hc != c {
            return false;
        }
    }

    true
}

fn rotate_letter(letter: char, n: usize) -> char {
    let mut unicode = letter as u32;
    for _ in 0..n {
        if unicode != ('z' as u32) {
            unicode += 1;
        } else {
            unicode = 'a' as u32;
        }
    }
    char::from_u32(unicode).unwrap()
}

fn decrypt_name(name: &str, sec_id: usize) -> String {
    name.chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                rotate_letter(c, sec_id)
            }
        })
        .collect()
}

fn sum_of_real_room_sector_ids(rooms: &str) -> usize {
    let re = Regex::new(REGEX).unwrap();

    rooms
        .lines()
        .map(|room| re.captures(room).unwrap())
        .filter(|caps| is_real_room(&caps[1], &caps[3]))
        .map(|caps| caps[2].parse::<usize>().unwrap())
        .sum()
}

fn decrypt_room_names(rooms: &str) -> Vec<(String, usize)> {
    let re = Regex::new(REGEX).unwrap();

    rooms
        .lines()
        .map(|room| re.captures(room).unwrap())
        .filter(|caps| is_real_room(&caps[1], &caps[3]))
        .map(|caps| {
            let sec_id = caps[2].parse::<usize>().unwrap();
            (decrypt_name(&caps[1], sec_id), sec_id)
        })
        .collect()
}

fn find_sector_id_for_room(room_name: &str, rooms: &str) -> usize {
    decrypt_room_names(&rooms)
        .iter()
        .filter(|(name, _)| name == room_name)
        .map(|(_, sec_id)| *sec_id)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{find_sector_id_for_room, sum_of_real_room_sector_ids};

    #[test]
    fn test_examples() {
        let rooms = "\
            aaaaa-bbb-z-y-x-123[abxyz]\n\
            a-b-c-d-e-f-g-h-987[abcde]\n\
            not-a-real-room-404[oarel]\n\
            totally-real-room-200[decoy]\
        ";

        assert_eq!(sum_of_real_room_sector_ids(rooms), 1514);
    }

    #[test]
    fn test_input() {
        let rooms = std::fs::read_to_string("input/rooms.txt").unwrap();

        assert_eq!(sum_of_real_room_sector_ids(&rooms), 185371);

        assert_eq!(
            find_sector_id_for_room("northpole object storage", &rooms),
            984
        );
    }
}
