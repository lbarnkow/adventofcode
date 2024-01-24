#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2020 - day 01");
}

fn part1(expenses: &str, target: usize) -> usize {
    let mut expenses: Vec<usize> = expenses
        .lines()
        .map(|line| line.parse::<usize>().expect("Should have been a number!"))
        .collect();
    expenses.sort();

    let mut a = 0;
    let mut b = expenses.len() - 1;

    while a < b {
        match (expenses[a] + expenses[b]).cmp(&target) {
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Less => a += 1,
            std::cmp::Ordering::Greater => b -= 1,
        }
    }

    expenses[a] * expenses[b]
}

fn part2(expenses: &str, target: usize) -> usize {
    let expenses: Vec<usize> = expenses.lines().map(|line| line.parse().unwrap()).collect();

    for a in 0..expenses.len() {
        for b in 0..expenses.len() {
            if b == a {
                continue;
            }
            for c in 0..expenses.len() {
                if c == a || c == b {
                    continue;
                }
                if expenses[a] + expenses[b] + expenses[c] == target {
                    return expenses[a] * expenses[b] * expenses[c];
                }
            }
        }
    }

    panic!("No possible solution!");
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_examples() {
        let expenses = "\
            1721\n\
            979\n\
            366\n\
            299\n\
            675\n\
            1456\
        ";

        let result = part1(expenses, 2020);
        assert_eq!(result, 514579);

        let result = part2(expenses, 2020);
        assert_eq!(result, 241861950);
    }

    #[test]
    fn test_input() {
        let expenses = std::fs::read_to_string("input/expenses.txt").unwrap();
        let result = part1(&expenses, 2020);
        assert_eq!(result, 878724);
        let result = part2(&expenses, 2020);
        assert_eq!(result, 201251610);
    }
}
