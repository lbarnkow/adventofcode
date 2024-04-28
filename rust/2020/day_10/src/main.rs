#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2020 - day 10");
}

fn parse_adapters(adapters: &str) -> Vec<u32> {
    let mut adapters: Vec<u32> = adapters
        .lines()
        .map(|line| line.parse().expect("Input is not a number!"))
        .collect();
    adapters.sort();
    adapters
}

fn differences_on_longest_chain(sorted_adapters: &[u32]) -> [u32; 3] {
    let mut prev_joltage = 0;
    let mut diff_counters = [0; 3];

    for adapter in sorted_adapters {
        let diff = *adapter - prev_joltage;
        if diff > 3 {
            panic!("Adapter {adapter} can't handle low joltage {prev_joltage} of previous step!");
        }

        diff_counters[(diff - 1) as usize] += 1;
        prev_joltage = *adapter;
    }

    diff_counters[2] += 1;
    diff_counters
}

fn count_viable_arrangements(sorted_adapters: &[u32]) -> usize {
    let mut buffer = vec![0; sorted_adapters.len()];
    buffer[sorted_adapters.len() - 1] = 1;

    for (idx, adapter) in sorted_adapters.iter().enumerate().rev() {
        for offset in 1..=3 {
            if offset > idx {
                continue;
            }

            sorted_adapters
                .get(idx - offset)
                .filter(|val| **val + 3 >= *adapter)
                .is_some()
                .then(|| buffer[idx - offset] += buffer[idx]);
        }
    }

    (0..=2).fold(0, |acc, idx| {
        acc + if sorted_adapters[idx] <= 3 {
            buffer[idx]
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{count_viable_arrangements, differences_on_longest_chain, parse_adapters};

    #[test]
    fn test_examples() {
        let adapters = "\
            16\n\
            10\n\
            15\n\
            5\n\
            1\n\
            11\n\
            7\n\
            19\n\
            6\n\
            12\n\
            4\
        ";
        let adapters = parse_adapters(adapters);

        let diffs = differences_on_longest_chain(&adapters);
        assert_eq!(diffs[0], 7);
        assert_eq!(diffs[2], 5);
        assert_eq!(diffs[0] * diffs[2], 35);

        let arrangements = count_viable_arrangements(&adapters);
        assert_eq!(arrangements, 8);

        let adapters = "\
            28\n\
            33\n\
            18\n\
            42\n\
            31\n\
            14\n\
            46\n\
            20\n\
            48\n\
            47\n\
            24\n\
            23\n\
            49\n\
            45\n\
            19\n\
            38\n\
            39\n\
            11\n\
            1\n\
            32\n\
            25\n\
            35\n\
            8\n\
            17\n\
            7\n\
            9\n\
            4\n\
            2\n\
            34\n\
            10\n\
            3\
        ";
        let adapters = parse_adapters(adapters);

        let diffs = differences_on_longest_chain(&adapters);
        assert_eq!(diffs[0], 22);
        assert_eq!(diffs[2], 10);
        assert_eq!(diffs[0] * diffs[2], 220);

        let arrangements = count_viable_arrangements(&adapters);
        assert_eq!(arrangements, 19208);
    }

    #[test]
    fn test_input() {
        let adapters = std::fs::read_to_string("input/adapters.txt").unwrap();
        let adapters = parse_adapters(&adapters);

        let diffs = differences_on_longest_chain(&adapters);
        assert_eq!(diffs[0], 68);
        assert_eq!(diffs[2], 28);
        assert_eq!(diffs[0] * diffs[2], 1904);

        let arrangements = count_viable_arrangements(&adapters);
        assert_eq!(arrangements, 10578455953408);
    }
}
