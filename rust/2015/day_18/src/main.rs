#![allow(dead_code)]
fn main() {
    println!("Advent of Code 2015 - day 18");
}

fn parse(width: usize, height: usize, lights: &str) -> Vec<bool> {
    assert_eq!(lights.len(), width * height);

    lights
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("illegal character"),
        })
        .collect()
}

fn count_lit_neighbors(x: usize, y: usize, width: usize, height: usize, lights: &Vec<bool>) -> u64 {
    let mut lit = 0;
    let x: i64 = x.try_into().unwrap();
    let y: i64 = y.try_into().unwrap();

    for _y in -1..=1 {
        for _x in -1..=1 {
            if _x == 0 && _y == 0 {
                continue;
            }

            let x: i64 = x + _x;
            let y: i64 = y + _y;
            if x < 0 || y < 0 || x >= width.try_into().unwrap() || y >= height.try_into().unwrap() {
                continue;
            }

            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();
            if lights[y * width + x] {
                lit += 1;
            }
        }
    }

    lit
}

fn is_corner(x: usize, y: usize, width: usize, height: usize) -> bool {
    (x == 0 && y == 0)
        || (x == 0 && y == height - 1)
        || (x == width - 1 && y == 0)
        || (x == width - 1 && y == height - 1)
}

fn step_lights(width: usize, height: usize, prev: &Vec<bool>, broken_corners: bool) -> Vec<bool> {
    let mut result = prev.clone();

    for y in 0..height {
        for x in 0..width {
            let lit = count_lit_neighbors(x, y, width, height, prev);

            if prev[y * width + x] {
                if lit != 2 && lit != 3 {
                    result[y * width + x] = false;
                }
            } else {
                if lit == 3 {
                    result[y * width + x] = true;
                }
            }

            if broken_corners && is_corner(x, y, width, height) {
                result[y * width + x] = true;
            }
        }
    }

    result
}

fn count_lit(lights: &Vec<bool>) -> u64 {
    lights.iter().map(|light| if *light { 1 } else { 0 }).sum()
}

fn to_string(lights: &Vec<bool>) -> String {
    lights.iter().map(|x| if *x { '#' } else { '.' }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{count_lit, parse, step_lights, to_string};

    #[test]
    fn test_examples() {
        let lights = ".#.#.#...##.#....#..#...#.#..#####..";
        let lights = parse(6, 6, lights);

        let lights = step_lights(6, 6, &lights, false);
        assert_eq!(to_string(&lights), "..##....##.#...##.......#.....#.##..");
        assert_eq!(count_lit(&lights), 11);

        let lights = step_lights(6, 6, &lights, false);
        assert_eq!(to_string(&lights), "..###.........###........#.....#....");
        assert_eq!(count_lit(&lights), 8);

        let lights = step_lights(6, 6, &lights, false);
        assert_eq!(to_string(&lights), "...#...........#....##..............");
        assert_eq!(count_lit(&lights), 4);

        let lights = step_lights(6, 6, &lights, false);
        assert_eq!(to_string(&lights), "..............##....##..............");
        assert_eq!(count_lit(&lights), 4);

        // broken corners
        let lights = "##.#.#...##.#....#..#...#.#..#####.#";
        let lights = parse(6, 6, lights);

        let lights = step_lights(6, 6, &lights, true);
        assert_eq!(to_string(&lights), "#.##.#####.#...##.......#...#.#.####");
        assert_eq!(count_lit(&lights), 18);

        let lights = step_lights(6, 6, &lights, true);
        assert_eq!(to_string(&lights), "#..#.##....#.#.##....##..#..####.###");
        assert_eq!(count_lit(&lights), 18);

        let lights = step_lights(6, 6, &lights, true);
        assert_eq!(to_string(&lights), "#...######.#..##.#......##....####.#");
        assert_eq!(count_lit(&lights), 18);

        let lights = step_lights(6, 6, &lights, true);
        assert_eq!(to_string(&lights), "#.#####....#...#...##...#.....#.#..#");
        assert_eq!(count_lit(&lights), 14);

        let lights = step_lights(6, 6, &lights, true);
        assert_eq!(to_string(&lights), "##.###.##..#.##....##...#.#...##...#");
        assert_eq!(count_lit(&lights), 17);
    }

    #[test]
    fn test_input() {
        let lights = std::fs::read_to_string("input/lights.txt")
            .unwrap()
            .replace('\n', "");
        let mut lights = parse(100, 100, &lights);

        for _ in 0..100 {
            lights = step_lights(100, 100, &lights, false);
        }

        assert_eq!(count_lit(&lights), 1061);

        let lights = std::fs::read_to_string("input/lights.txt")
            .unwrap()
            .replace('\n', "");
        let mut lights = parse(100, 100, &lights);

        for _ in 0..100 {
            lights = step_lights(100, 100, &lights, true);
        }

        assert_eq!(count_lit(&lights), 1006);
    }
}
