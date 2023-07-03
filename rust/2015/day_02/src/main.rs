use std::cmp::min;

fn main() {
    println!("Advent of Code 2015 - day 2");
}

fn calculate_required_wrapping_paper(l: usize, w: usize, h: usize) -> usize {
    let a = l * w;
    let b = w * h;
    let c = l * h;
    let smallest_side = min(a, min(b, c));

    2 * a + 2 * b + 2 * c + smallest_side
}

fn calculate_required_ribbon(l: usize, w: usize, h: usize) -> usize {
    let mut dimensions = [l, w, h];
    dimensions.sort();

    let ribbon = dimensions.get(0).unwrap() * 2 + dimensions.get(1).unwrap() * 2;
    let bow_slack = l * w * h;

    ribbon + bow_slack
}

#[cfg(test)]
mod tests {
    use crate::{calculate_required_ribbon, calculate_required_wrapping_paper};

    #[test]
    fn test_examples() {
        assert_eq!(calculate_required_wrapping_paper(2, 3, 4), 58);
        assert_eq!(calculate_required_wrapping_paper(1, 1, 10), 43);

        assert_eq!(calculate_required_ribbon(2, 3, 4), 34);
        assert_eq!(calculate_required_ribbon(1, 1, 10), 14);
    }

    fn parse_line(line: &str) -> (usize, usize, usize) {
        let parts: Vec<&str> = line.split('x').collect();

        let l = usize::from_str_radix(parts.get(0).unwrap(), 10).unwrap();
        let w = usize::from_str_radix(parts.get(1).unwrap(), 10).unwrap();
        let h = usize::from_str_radix(parts.get(2).unwrap(), 10).unwrap();

        (l, w, h)
    }

    #[test]
    fn test_input() {
        let lines = std::fs::read_to_string("input/sizes.txt").unwrap();

        let mut total_paper = 0;
        let mut total_ribbon = 0;
        for line in lines.lines() {
            let (l, w, h) = parse_line(line);
            total_paper += calculate_required_wrapping_paper(l, w, h);
            total_ribbon += calculate_required_ribbon(l, w, h);
        }

        assert_eq!(total_paper, 1606483);
        assert_eq!(total_ribbon, 3842356);
    }
}
