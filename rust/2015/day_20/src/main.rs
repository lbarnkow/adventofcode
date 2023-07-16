#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2015 - day 20");
}

fn lowest_house_number_with_n_presents(n: usize) -> usize {
    let upper_bound = usize::max(n / 10, 1);
    let mut scores = vec![0; upper_bound + 1];

    for elf in 1..=upper_bound {
        for house in (elf..=upper_bound).step_by(elf) {
            scores[house] += elf * 10;
        }
    }

    for house in 1..=upper_bound {
        if scores[house] >= n {
            return house;
        }
    }

    panic!("Impossible!");
}

fn lowest_house_number_with_n_presents_lazy(n: usize) -> usize {
    let upper_bound = usize::max(n / 11, 1);
    let mut scores = vec![0; upper_bound + 1];

    for elf in 1..=upper_bound {
        let max_house = usize::min(elf * 50, upper_bound);
        for house in (elf..=max_house).step_by(elf) {
            scores[house] += elf * 11;
        }
    }

    for house in 1..=upper_bound {
        if scores[house] >= n {
            return house;
        }
    }

    panic!("Impossible!");
}

#[cfg(test)]
mod tests {
    use crate::{lowest_house_number_with_n_presents, lowest_house_number_with_n_presents_lazy};

    #[test]
    fn test_examples() {
        assert_eq!(lowest_house_number_with_n_presents(10), 1);
        assert_eq!(lowest_house_number_with_n_presents(20), 2);
        assert_eq!(lowest_house_number_with_n_presents(30), 2);
        assert_eq!(lowest_house_number_with_n_presents(40), 3);
        assert_eq!(lowest_house_number_with_n_presents(50), 4);
        assert_eq!(lowest_house_number_with_n_presents(60), 4);
        assert_eq!(lowest_house_number_with_n_presents(70), 4);
        assert_eq!(lowest_house_number_with_n_presents(80), 6);
        assert_eq!(lowest_house_number_with_n_presents(90), 6);
        assert_eq!(lowest_house_number_with_n_presents(100), 6);
        assert_eq!(lowest_house_number_with_n_presents(110), 6);
        assert_eq!(lowest_house_number_with_n_presents(120), 6);
        assert_eq!(lowest_house_number_with_n_presents(130), 8);
        assert_eq!(lowest_house_number_with_n_presents(140), 8);
        assert_eq!(lowest_house_number_with_n_presents(150), 8);
    }

    #[test]
    fn test_input() {
        assert_eq!(lowest_house_number_with_n_presents(36_000_000), 831_600);
        assert_eq!(
            lowest_house_number_with_n_presents_lazy(36_000_000),
            884_520
        );
    }
}
