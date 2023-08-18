#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display, usize};

fn main() {
    println!("Advent of Code 2017 - day 21");
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Pixel {
    On,
    Off,
}

impl From<char> for Pixel {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Off,
            '#' => Self::On,
            _ => panic!("Illegal pixel: {value}!"),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pixel::On => write!(f, "#"),
            Pixel::Off => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Grid {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split("/").collect();
        let height = split.len();
        let width = height;

        let mut pixels = Vec::with_capacity(width * height);

        for row in split {
            for pixel in row.chars() {
                pixels.push(pixel.into());
            }
        }

        assert_eq!(pixels.len(), width * height);

        Self {
            width,
            height,
            pixels,
        }
    }
}

impl From<&Vec<Grid>> for Grid {
    fn from(grids: &Vec<Grid>) -> Self {
        let num_grids = grids.len() as f64;
        let num_grids_h: usize = num_grids.sqrt() as usize;

        let sub_width = grids[0].width;
        let width = num_grids_h * sub_width;

        let mut pixels = Vec::new();

        for meta_row in 0..num_grids_h {
            for sub_row in 0..sub_width {
                for meta_col in 0..num_grids_h {
                    for sub_col in 0..sub_width {
                        let sub_grid_idx = meta_row * num_grids_h + meta_col;
                        let sub_grid = &grids[sub_grid_idx];
                        let pxl_idx = sub_row * sub_width + sub_col;
                        let pxl = sub_grid.pixels[pxl_idx];
                        pixels.push(pxl);
                    }
                }
            }
        }

        Self {
            width,
            height: width,
            pixels,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.pixels[row * self.width + col]).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

impl Grid {
    fn count_pixels_matching(&self, state: Pixel) -> usize {
        self.pixels.iter().filter(|pixel| **pixel == state).count()
    }

    fn enhance(&self, rules: &Vec<Rule>) -> Self {
        let split = self.split();
        let mut enhanced = Vec::with_capacity(split.len());

        for grid in split {
            let mut matched = false;
            for rule in rules {
                if rule.matches(&grid) {
                    matched = true;
                    enhanced.push(rule.to.clone());
                }
            }
            if !matched {
                panic!("No rule matched this grid!");
            }
        }

        Grid::from(&enhanced)
    }

    fn split(&self) -> Vec<Self> {
        let (num_grids_h, sub_width) = if self.width % 2 == 0 {
            (self.width / 2, 2)
        } else if self.width % 3 == 0 {
            (self.width / 3, 3)
        } else {
            panic!("Illegal grid size!")
        };
        let mut grids = Vec::with_capacity(num_grids_h * num_grids_h);

        for sub_row in 0..num_grids_h {
            for sub_col in 0..num_grids_h {
                let mut pixels = Vec::with_capacity(sub_width * sub_width);
                let base_idx = sub_row * (self.width * sub_width) + sub_col * sub_width;
                for row in 0..sub_width {
                    for col in 0..sub_width {
                        let sub_idx = row * self.width + col;
                        pixels.push(self.pixels[base_idx + sub_idx]);
                    }
                }
                grids.push(Self {
                    width: sub_width,
                    height: sub_width,
                    pixels,
                })
            }
        }

        grids
    }

    fn rotate_cw(&self) -> Self {
        let mut pixels = Vec::with_capacity(self.pixels.len());

        for col in 0..self.width {
            for row in (0..self.height).rev() {
                pixels.push(self.pixels[row * self.width + col]);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    fn flip_h(&self) -> Self {
        let mut pixels = Vec::with_capacity(self.pixels.len());

        for row in 0..self.height {
            for col in (0..self.width).rev() {
                pixels.push(self.pixels[row * self.width + col]);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    fn flip_v(&self) -> Self {
        let mut pixels = Vec::with_capacity(self.pixels.len());

        for row in (0..self.height).rev() {
            for col in 0..self.width {
                pixels.push(self.pixels[row * self.width + col]);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            pixels,
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    raw: String,
    width: usize,
    from: HashSet<Grid>,
    to: Grid,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut split = value.split(" => ");

        let mut base_from = Grid::from(split.next().unwrap());
        let to = Grid::from(split.next().unwrap());

        let mut from = HashSet::new();
        from.insert(base_from.clone());
        from.insert(base_from.flip_h());
        from.insert(base_from.flip_v());
        for _ in 0..3 {
            base_from = base_from.rotate_cw();
            from.insert(base_from.clone());
            from.insert(base_from.flip_h());
            from.insert(base_from.flip_v());
        }

        Self {
            raw: value.to_string(),
            width: base_from.width,
            from,
            to,
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.raw)
    }
}

impl Rule {
    fn from_multiple(value: &str) -> Vec<Rule> {
        value.lines().map(|line| line.into()).collect()
    }

    fn matches(&self, grid: &Grid) -> bool {
        self.from.contains(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Grid, Pixel, Rule};

    #[test]
    fn test_examples() {
        let rules = "\
            ../.# => ##./#../...\n\
            .#./..#/### => #..#/..../..../#..#\
        ";
        let rules = Rule::from_multiple(rules);

        let raw_grid = "\
            .#.\n\
            ..#\n\
            ###\
        ";
        let grid = Grid::from(raw_grid.replace("\n", "/").as_str());

        let grid = grid.enhance(&rules);
        let expected = "\
            #..#\n\
            ....\n\
            ....\n\
            #..#\n\
        ";
        assert_eq!(grid.to_string(), expected);
        assert_eq!(grid.count_pixels_matching(Pixel::Off), 12);
        assert_eq!(grid.count_pixels_matching(Pixel::On), 4);

        let grid = grid.enhance(&rules);
        let expected = "\
            ##.##.\n\
            #..#..\n\
            ......\n\
            ##.##.\n\
            #..#..\n\
            ......\n\
        ";
        assert_eq!(grid.to_string(), expected);
        assert_eq!(grid.count_pixels_matching(Pixel::Off), 24);
        assert_eq!(grid.count_pixels_matching(Pixel::On), 12);
    }

    #[test]
    fn test_input() {
        let rules = std::fs::read_to_string("input/enhancements.txt").unwrap();
        let rules = Rule::from_multiple(rules.as_str());

        let raw_grid = "\
            .#.\n\
            ..#\n\
            ###\
        ";
        let mut grid = Grid::from(raw_grid.replace("\n", "/").as_str());

        for _ in 0..5 {
            grid = grid.enhance(&rules);
        }

        assert_eq!(grid.count_pixels_matching(Pixel::On), 203);
    }

    #[test]
    fn test_input_part2() {
        let rules = std::fs::read_to_string("input/enhancements.txt").unwrap();
        let rules = Rule::from_multiple(rules.as_str());

        let raw_grid = "\
            .#.\n\
            ..#\n\
            ###\
        ";
        let mut grid = Grid::from(raw_grid.replace("\n", "/").as_str());

        // takes ~5s on a mobile Core i7 from 2016, good enough
        for _ in 0..18 {
            grid = grid.enhance(&rules);
        }

        assert_eq!(grid.count_pixels_matching(Pixel::On), 3342470);
    }
}
