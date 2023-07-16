#![allow(dead_code)]
fn main() {
    println!("Advent of Code 2015 - day 17");
}

fn parse_containers(containers: &str) -> Vec<u64> {
    containers
        .lines()
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect()
}

fn compute_possible_combinations_rec(
    combinations: &mut Vec<Vec<u64>>,
    current_stack: &mut Vec<u64>,
    amount: u64,
    containers: &[u64],
) {
    if containers.len() == 1 {
        if amount == 0 {
            combinations.push(current_stack.clone());
        } else if amount == containers[0] {
            current_stack.push(containers[0]);
            combinations.push(current_stack.clone());
            current_stack.pop();
        }
        return;
    }

    if containers[0] <= amount {
        current_stack.push(containers[0]);
        compute_possible_combinations_rec(
            combinations,
            current_stack,
            amount - containers[0],
            &containers[1..],
        );
        current_stack.pop();
    }

    compute_possible_combinations_rec(combinations, current_stack, amount, &containers[1..]);
}

fn compute_possible_combinations(amount: u64, containers: &str) -> Vec<Vec<u64>> {
    let mut containers = parse_containers(containers);
    containers.sort();
    containers.reverse();

    let mut result = Vec::new();
    compute_possible_combinations_rec(&mut result, &mut Vec::new(), amount, &containers);
    result
}

fn compute_number_of_combinations_using_the_minimum_number_of_containers(
    combinations: &Vec<Vec<u64>>,
) -> usize {
    let min = combinations
        .iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();
    combinations
        .iter()
        .filter(|combination| combination.len() == min)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{
        compute_number_of_combinations_using_the_minimum_number_of_containers,
        compute_possible_combinations,
    };

    #[test]
    fn test_examples() {
        let containers = "\
            20\n\
            15\n\
            10\n\
            5\n\
            5\
        ";

        let combinations = compute_possible_combinations(25, containers);
        println!("{combinations:?}");
        assert_eq!(combinations.len(), 4);

        let min_combinations =
            compute_number_of_combinations_using_the_minimum_number_of_containers(&combinations);
        assert_eq!(min_combinations, 3);
    }

    #[test]
    fn test_input() {
        let containers = std::fs::read_to_string("input/containers.txt").unwrap();
        let combinations = compute_possible_combinations(150, &containers);
        println!("{combinations:?}");
        assert_eq!(combinations.len(), 4372);

        let min_combinations =
            compute_number_of_combinations_using_the_minimum_number_of_containers(&combinations);
        assert_eq!(min_combinations, 4);
    }
}
