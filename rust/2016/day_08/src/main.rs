#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2016 - day 08");
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Rect { width: usize, height: usize },
    RotateRow { row: usize, steps: usize },
    RotateCol { col: usize, steps: usize },
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.starts_with("rect ") {
            let mut iter = (&value[5..])
                .split('x')
                .map(|s| s.parse::<usize>().unwrap());
            return Self::Rect {
                width: iter.next().unwrap(),
                height: iter.next().unwrap(),
            };
        } else if value.starts_with("rotate row y=") {
            let mut iter = (&value[13..])
                .split(" by ")
                .map(|s| s.parse::<usize>().unwrap());
            return Self::RotateRow {
                row: iter.next().unwrap(),
                steps: iter.next().unwrap(),
            };
        } else if value.starts_with("rotate column x=") {
            let mut iter = (&value[16..])
                .split(" by ")
                .map(|s| s.parse::<usize>().unwrap());
            return Self::RotateCol {
                col: iter.next().unwrap(),
                steps: iter.next().unwrap(),
            };
        } else {
            panic!("Illegal instruction: {value}");
        }
    }
}

impl Instruction {
    fn from_lines(lines: &str) -> Vec<Self> {
        lines.lines().map(|line| Self::from(line)).collect()
    }
}

#[derive(Debug)]
struct Screen {
    cols: usize,
    rows: usize,
    pixels: Vec<bool>,
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                let px = if self.pixels[y * self.cols + x] {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{px}")?;
            }
            write!(f, "\n")?;
        }

        Result::Ok(())
    }
}

impl Screen {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            pixels: vec![false; cols * rows],
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                self.pixels[y * self.cols + x] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, steps: usize) {
        for _ in 0..steps {
            let buffer = self.pixels[row * self.cols + self.cols - 1];
            for col in (1..self.cols).rev() {
                self.pixels[row * self.cols + col] = self.pixels[row * self.cols + col - 1];
            }
            self.pixels[row * self.cols] = buffer;
        }
    }

    fn rotate_col(&mut self, col: usize, steps: usize) {
        for _ in 0..steps {
            let buffer = self.pixels[(self.rows - 1) * self.cols + col];
            for row in (1..self.rows).rev() {
                self.pixels[row * self.cols + col] = self.pixels[(row - 1) * self.cols + col];
            }
            self.pixels[col] = buffer;
        }
    }

    fn step(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rect { width, height } => self.rect(*width, *height),
            Instruction::RotateRow { row, steps } => self.rotate_row(*row, *steps),
            Instruction::RotateCol { col, steps } => self.rotate_col(*col, *steps),
        }
        println!("{instruction:?}");
        println!("{self}");
    }

    fn step_multiple(&mut self, instructions: &Vec<Instruction>) {
        instructions.iter().for_each(|i| self.step(i));
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().filter(|px| **px).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, Screen};

    #[test]
    fn test_instruction_from_str() {
        let instructions = "\
            rect 3x2\n\
            rotate column x=1 by 1\n\
            rotate row y=0 by 4\
        ";

        let expected = vec![
            Instruction::Rect {
                width: 3,
                height: 2,
            },
            Instruction::RotateCol { col: 1, steps: 1 },
            Instruction::RotateRow { row: 0, steps: 4 },
        ];

        instructions
            .lines()
            .enumerate()
            .for_each(|(i, line)| assert_eq!(Instruction::from(line), expected[i]));

        assert_eq!(Instruction::from_lines(instructions), expected);
    }

    #[test]
    fn test_steps() {
        let mut screen = Screen::new(7, 3);

        let expected = "\
            .......\n\
            .......\n\
            .......\n\
        ";
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 0);

        screen.step(&Instruction::Rect {
            width: 3,
            height: 2,
        });
        let expected = "\
            ###....\n\
            ###....\n\
            .......\n\
        ";
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);

        screen.step(&Instruction::RotateCol { col: 1, steps: 1 });
        let expected = "\
            #.#....\n\
            ###....\n\
            .#.....\n\
        ";
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);

        screen.step(&Instruction::RotateCol { col: 1, steps: 3 });
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);

        screen.step(&Instruction::RotateRow { row: 0, steps: 4 });
        let expected = "\
            ....#.#\n\
            ###....\n\
            .#.....\n\
        ";
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);

        screen.step(&Instruction::RotateRow { row: 0, steps: 7 });
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);

        screen.step(&Instruction::RotateCol { col: 1, steps: 1 });
        let expected = "\
            .#..#.#\n\
            #.#....\n\
            .#.....\n\
        ";
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);
    }

    #[test]
    fn test_example() {
        let instructions = "\
            rect 3x2\n\
            rotate column x=1 by 1\n\
            rotate row y=0 by 4\n\
            rotate column x=1 by 1\
        ";
        let instructions = Instruction::from_lines(instructions);

        let expected = "\
            .#..#.#\n\
            #.#....\n\
            .#.....\n\
        ";

        let mut screen = Screen::new(7, 3);
        screen.step_multiple(&instructions);
        assert_eq!(screen.to_string(), expected);
        assert_eq!(screen.count_lit(), 6);
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();
        let instructions = Instruction::from_lines(&instructions);

        let mut screen = Screen::new(50, 6);
        println!("{screen}");
        screen.step_multiple(&instructions);
        assert_eq!(screen.count_lit(), 106);

        let expected = "\
            .##..####.#....####.#.....##..#...#####..##...###.\n\
            #..#.#....#....#....#....#..#.#...##....#..#.#....\n\
            #....###..#....###..#....#..#..#.#.###..#....#....\n\
            #....#....#....#....#....#..#...#..#....#.....##..\n\
            #..#.#....#....#....#....#..#...#..#....#..#....#.\n\
            .##..#....####.####.####..##....#..#.....##..###..\n\
        "; // CFLELOYFCS
        assert_eq!(screen.to_string(), expected);
    }
}
