#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 03");
}

fn has_valid_sides(a: usize, b: usize, c: usize) -> Result<(), &'static str> {
    if a + b > c && a + c > b && b + c > a {
        Ok(())
    } else {
        Err("the sum of any two sides must be larger than the remaining side")
    }
}

fn count_possible_triangles(input: &str) -> usize {
    let mut count = 0;

    for line in input.lines() {
        let triangle: Vec<usize> = line
            .trim()
            .split(' ')
            .filter(|split| !split.is_empty())
            .map(|side| side.parse().unwrap())
            .collect();
        assert_eq!(triangle.len(), 3);
        if let Ok(()) = has_valid_sides(triangle[0], triangle[1], triangle[2]) {
            count += 1;
        }
    }

    count
}

fn count_possible_triangles_v(input: &str) -> usize {
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(' ')
                .filter(|split| !split.is_empty())
                .map(|side| side.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let rows = input.len();
    let cols = (&input[0]).len();

    let mut count = 0;

    for col in 0..cols {
        for row in (0..rows).step_by(3) {
            let (a, b, c) = (input[row][col], input[row + 1][col], input[row + 2][col]);
            if let Ok(()) = has_valid_sides(a, b, c) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{count_possible_triangles, count_possible_triangles_v, has_valid_sides};

    #[test]
    fn test_examples() {
        assert!(has_valid_sides(1, 1, 1).is_ok());
        assert!(has_valid_sides(5, 10, 25).is_err());
        assert!(has_valid_sides(10, 10, 14).is_ok());

        let input = "\
            1 1 1\n\
            5 10 25\n\
            10 10 14\
        ";

        assert_eq!(count_possible_triangles(input), 2);
        assert_eq!(count_possible_triangles_v(input), 1);

        let input = "\
            101 301 501\n\
            102 302 502\n\
            103 303 503\n\
            201 401 601\n\
            202 402 602\n\
            203 403 603\
        ";

        assert_eq!(count_possible_triangles(input), 3);
        assert_eq!(count_possible_triangles_v(input), 6);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input/triangles.txt").unwrap();

        assert_eq!(count_possible_triangles(&input), 869);
        assert_eq!(count_possible_triangles_v(&input), 1544);
    }
}
