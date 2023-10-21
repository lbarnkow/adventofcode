#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2019 - day 08");
}

struct Layer {
    data: Vec<char>,
}

impl From<&str> for Layer {
    fn from(value: &str) -> Self {
        Self {
            data: value.chars().collect(),
        }
    }
}

impl Layer {
    fn count(&self, c: char) -> usize {
        self.data.iter().filter(|d| **d == c).count()
    }
}

enum Color {
    Black,
    White,
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Black,
            '1' => Self::White,
            x => panic!("Illegal color: {x}!"),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Color::Black => "⬛️",
            Color::White => "⬜️",
        };
        write!(f, "{c}")
    }
}

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize, data: &str) -> Self {
        let layer_len = width * height;
        assert_eq!(data.len() % layer_len, 0);

        let layer_count = data.len() / layer_len;
        let mut layers = Vec::with_capacity(layer_count);

        for i in 0..layer_count {
            let start = i * layer_len;
            let end = start + layer_len;
            let sub_slice = &data[start..end];
            layers.push(Layer::from(sub_slice));
        }

        Self {
            layers,
            width,
            height,
        }
    }

    fn checksum(&self, least_common: char, count_1: char, count_2: char) -> usize {
        let least_common = self
            .layers
            .iter()
            .min_by_key(|l| l.count(least_common))
            .unwrap();

        least_common.count(count_1) * least_common.count(count_2)
    }

    fn flatten(&self) -> Vec<Vec<Color>> {
        let mut rows = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                for layer in &self.layers {
                    if layer.data[y * self.width + x] != '2' {
                        row.push(layer.data[y * self.width + x].into());
                        break;
                    }
                }
            }
            rows.push(row);
        }
        rows
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flattened = self.flatten();
        let mut sep = "";

        for row in flattened {
            write!(f, "{sep}").unwrap();
            for pixel in row {
                write!(f, "{}", pixel).unwrap();
            }
            sep = "\n";
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Image;

    #[test]
    fn test_examples() {
        let data = "123456789012";
        let image = Image::new(3, 2, data);

        assert_eq!(image.layers.len(), 2);

        let checksum = image.checksum('0', '1', '2');
        assert_eq!(checksum, 1);
    }

    #[test]
    fn test_input() {
        let data = std::fs::read_to_string("input/image.txt").unwrap();
        let image = Image::new(25, 6, data.as_str());

        assert_eq!(image.layers.len(), 100);

        let checksum = image.checksum('0', '1', '2');
        assert_eq!(checksum, 1935);

        let expected = "\
            ⬛️⬜️⬜️⬛️⬛️⬜️⬜️⬜️⬜️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️\n\
            ⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️\n\
            ⬜️⬛️⬛️⬛️⬛️⬜️⬜️⬜️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️\n\
            ⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️\n\
            ⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬛️⬛️⬜️⬛️⬜️⬛️⬛️⬛️⬛️\n\
            ⬛️⬜️⬜️⬛️⬛️⬜️⬛️⬛️⬛️⬛️⬜️⬜️⬜️⬜️⬛️⬛️⬜️⬜️⬛️⬛️⬜️⬜️⬜️⬜️⬛️\
        ";
        let actual = image.to_string();
        assert_eq!(actual, expected);
    }
}
