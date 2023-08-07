#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 02");
}

fn row_min_max_checksum(line: &str) -> u32 {
    let mut min = u32::MAX;
    let mut max = u32::MIN;

    for num in line.split("\t") {
        let num = num.parse().unwrap();
        min = min.min(num);
        max = max.max(num);
    }

    max - min
}

fn sheet_checksum_min_max(input: &str) -> u32 {
    input.lines().map(|line| row_min_max_checksum(line)).sum()
}

fn row_evenly_divisible_checksum(line: &str) -> u32 {
    let split: Vec<&str> = line.split("\t").collect();

    for (idx, a) in split.iter().enumerate() {
        let a: u32 = a.parse().unwrap();
        for b in split[idx + 1..].iter() {
            let b: u32 = b.parse().unwrap();
            let min = a.min(b);
            let max = a.max(b);

            if max % min == 0 {
                return max / min;
            }
        }
    }

    0
}

fn sheet_checksum_evenly_divisible(input: &str) -> u32 {
    input
        .lines()
        .map(|line| row_evenly_divisible_checksum(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        row_evenly_divisible_checksum, row_min_max_checksum, sheet_checksum_evenly_divisible,
        sheet_checksum_min_max,
    };

    #[test]
    fn test_examples() {
        assert_eq!(row_min_max_checksum("5\t1\t9\t5"), 8);
        assert_eq!(row_min_max_checksum("7\t5\t3"), 4);
        assert_eq!(row_min_max_checksum("2\t4\t6\t8"), 6);

        let input = "\
            5\t1\t9\t5\n\
            7\t5\t3\n\
            2\t4\t6\t8\
        ";

        assert_eq!(sheet_checksum_min_max(input), 18);

        assert_eq!(row_evenly_divisible_checksum("5\t9\t2\t8"), 4);
        assert_eq!(row_evenly_divisible_checksum("9\t4\t7\t3"), 3);
        assert_eq!(row_evenly_divisible_checksum("3\t8\t6\t5"), 2);

        let input = "\
            5\t9\t2\t8\n\
            9\t4\t7\t3\n\
            3\t8\t6\t5\
        ";

        assert_eq!(sheet_checksum_evenly_divisible(input), 9);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/spreadsheet.txt").unwrap();

        assert_eq!(sheet_checksum_min_max(&input), 47623);
        assert_eq!(sheet_checksum_evenly_divisible(&input), 312);
    }
}
