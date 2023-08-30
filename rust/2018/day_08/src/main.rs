#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2018 - day 08");
}

fn compute_metadata_sum<'a, T>(iter: &mut T) -> u64
where
    T: Iterator<Item = &'a str>,
{
    let n_children: usize = iter.next().unwrap().parse().unwrap();
    let n_metadata: usize = iter.next().unwrap().parse().unwrap();

    let mut sum = 0;

    for _ in 0..n_children {
        sum += compute_metadata_sum(iter);
    }

    for _ in 0..n_metadata {
        sum += iter.next().unwrap().parse::<u64>().unwrap();
    }

    sum
}

fn compute_node_value<'a, T>(iter: &mut T) -> u64
where
    T: Iterator<Item = &'a str>,
{
    let n_children: usize = iter.next().unwrap().parse().unwrap();
    let n_metadata: usize = iter.next().unwrap().parse().unwrap();

    let children: Vec<u64> = (0..n_children).map(|_| compute_node_value(iter)).collect();
    let metadata: Vec<u64> = (0..n_metadata)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    if children.is_empty() {
        return metadata.into_iter().sum();
    } else {
        return metadata
            .into_iter()
            .filter(|rf| *rf > 0 && *rf <= children.len() as u64)
            .map(|rf| children[rf as usize - 1])
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use crate::{compute_metadata_sum, compute_node_value};

    #[test]
    fn test_examples() {
        let tree = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        let metadata_sum = compute_metadata_sum(&mut tree.split(" "));
        assert_eq!(metadata_sum, 138);

        let node_value = compute_node_value(&mut tree.split(" "));
        assert_eq!(node_value, 66);
    }

    #[test]
    fn test_input() {
        let tree = std::fs::read_to_string("input/tree.txt").unwrap();

        let metadata_sum = compute_metadata_sum(&mut tree.split(" "));
        assert_eq!(metadata_sum, 42798);

        let node_value = compute_node_value(&mut tree.split(" "));
        assert_eq!(node_value, 23798);
    }
}
