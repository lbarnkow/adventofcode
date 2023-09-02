#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2018 - day 11");
}

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn compute_power_level(x: i64, y: i64, sn: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += sn;
    power_level *= rack_id;
    power_level = if power_level < 100 {
        0
    } else {
        (power_level / 100) % 10
    };
    power_level - 5
}

fn fill_map(map: &mut [i64], sn: i64) {
    for y in 1..=HEIGHT {
        for x in 1..=WIDTH {
            map[(y - 1) * WIDTH + (x - 1)] = compute_power_level(x as i64, y as i64, sn);
        }
    }
}

fn find_strongest_cluster(map: &[i64], sq_size: usize) -> (i64, usize, usize) {
    let mut strongest = i64::MIN;
    let mut id = (1, 1, 1);

    for y in 1..=(HEIGHT - (sq_size - 1)) {
        for x in 1..=(WIDTH - (sq_size - 1)) {
            let mut power = 0;
            for y_off in 0..sq_size {
                for x_off in 0..sq_size {
                    power += map[(y + y_off - 1) * WIDTH + (x + x_off - 1)];
                }
            }

            if power > strongest {
                strongest = power;
                id = (power, x, y);
            }
        }
    }

    id
}

fn find_strongest_cluster_any_size(map: &[i64]) -> (usize, usize, usize) {
    let mut strongest = i64::MIN;
    let mut id = (1, 1, 1);
    let mut worse_counter = 0;

    for sq_size in 1..=WIDTH {
        let (power, x, y) = find_strongest_cluster(map, sq_size);
        if power > strongest {
            strongest = power;
            id = (x, y, sq_size);
        } else {
            worse_counter += 1;
        }

        // arbitrary short-cut to not grow any farther, after
        // results are getting worse to speed things up.
        // the overall approach is still brute force.
        // takes < 1s on a mobile Core i7 dual core from 2016.
        if worse_counter == 10 {
            break;
        }
    }

    id
}

#[cfg(test)]
mod tests {
    use crate::{
        compute_power_level, fill_map, find_strongest_cluster, find_strongest_cluster_any_size,
        HEIGHT, WIDTH,
    };

    #[test]
    fn test_examples() {
        assert_eq!(compute_power_level(3, 5, 8), 4);
        assert_eq!(compute_power_level(122, 79, 57), -5);
        assert_eq!(compute_power_level(217, 196, 39), 0);
        assert_eq!(compute_power_level(101, 153, 71), 4);

        let mut map = [0i64; WIDTH * HEIGHT];
        fill_map(&mut map, 18);

        assert_eq!(find_strongest_cluster(&map, 3), (29, 33, 45));
        assert_eq!(find_strongest_cluster_any_size(&map), (90, 269, 16));

        fill_map(&mut map, 42);
        assert_eq!(find_strongest_cluster(&map, 3), (30, 21, 61));
        assert_eq!(find_strongest_cluster_any_size(&map), (232, 251, 12));
    }

    #[test]
    fn test_input() {
        let mut map = [0i64; WIDTH * HEIGHT];
        fill_map(&mut map, 7165);
        assert_eq!(find_strongest_cluster(&map, 3), (31, 235, 20));
        assert_eq!(find_strongest_cluster_any_size(&map), (237, 223, 14));
    }
}
