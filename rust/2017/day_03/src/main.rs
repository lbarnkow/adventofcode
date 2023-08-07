#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2017 - day 03");
}

fn ring_max(ring: usize) -> usize {
    ((ring - 1) * 2 + 1) * ((ring - 1) * 2 + 1)
}

fn find_ring(input: usize) -> usize {
    let mut ring = 1;
    loop {
        if input <= ring_max(ring) {
            return ring;
        }
        ring += 1;
    }
}

fn manhattan_distance_to_center(input: usize) -> usize {
    if input == 1 {
        return 0;
    }

    let ring = find_ring(input);
    let ring_max = ring_max(ring);
    let ring_side_len = (ring - 1) * 2 + 1;
    let max_additional_steps = ring_side_len / 2;

    let mut go_down = true;
    let mut additional_steps = max_additional_steps;

    let mut item_on_ring = ring_max;

    loop {
        if item_on_ring == input {
            break;
        }
        item_on_ring -= 1;

        if go_down {
            additional_steps -= 1;
            if additional_steps == 0 {
                go_down = false
            }
        } else {
            additional_steps += 1;
            if additional_steps == max_additional_steps {
                go_down = true;
            }
        }
    }

    ring - 1 + additional_steps
}

fn pos_to_idx((x, y): (usize, usize), side_len: usize) -> usize {
    y * side_len + x
}

fn neighbors((x, y): (usize, usize)) -> [(usize, usize); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

#[derive(Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (x, y - 1),
            Dir::Left => (x - 1, y),
            Dir::Down => (x, y + 1),
            Dir::Right => (x + 1, y),
        }
    }
}

fn print_spiral(spiral: &Vec<usize>, side_len: usize) {
    for y in 0..side_len {
        for x in 0..side_len {
            print!("{:^9}", spiral[pos_to_idx((x, y), side_len)]);
        }
        println!("");
    }
}

fn spiral_part_2(max_side_len: usize, value: usize) -> usize {
    if value == 0 {
        return 1;
    }

    let dim = max_side_len;
    let mut spiral: Vec<usize> = vec![0; dim * dim];

    let (mut x, mut y) = (dim / 2, dim / 2);
    spiral[pos_to_idx((x, y), dim)] = 1;

    let mut ring = 1;
    loop {
        ring += 1;
        let ring_side_len = (ring - 1) * 2 + 1;
        (x, y) = (x + 1, y + 1);

        for dir in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
            for _ in 0..(ring_side_len - 1) {
                (x, y) = dir.step((x, y));
                let sum: usize = neighbors((x, y))
                    .into_iter()
                    .map(|(x, y)| spiral[pos_to_idx((x, y), dim)])
                    .sum();
                spiral[pos_to_idx((x, y), dim)] = sum;
                if sum > value {
                    print_spiral(&spiral, dim);
                    return sum;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{manhattan_distance_to_center, spiral_part_2};

    #[test]
    fn test_examples() {
        assert_eq!(manhattan_distance_to_center(1), 0);
        assert_eq!(manhattan_distance_to_center(12), 3);
        assert_eq!(manhattan_distance_to_center(23), 2);
        assert_eq!(manhattan_distance_to_center(1024), 31);

        assert_eq!(spiral_part_2(11, 50), 54);
        assert_eq!(spiral_part_2(11, 100), 122);
        assert_eq!(spiral_part_2(11, 300), 304);
        assert_eq!(spiral_part_2(11, 800), 806);
    }

    #[test]
    fn test_input() {
        assert_eq!(manhattan_distance_to_center(312051), 430);

        assert_eq!(spiral_part_2(11, 312051), 312453);
    }
}
