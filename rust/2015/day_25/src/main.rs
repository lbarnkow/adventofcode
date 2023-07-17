#![allow(dead_code)]

use std::fmt::Display;
fn main() {
    println!("Advent of Code 2015 - day 25");
}

#[derive(Debug)]
struct CodeTable {
    rows: usize,
    cols: usize,
    data: Vec<u128>,
}

impl Display for CodeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CodeTable({}, {})", self.cols, self.rows).unwrap();

        let max_len = self.data.iter().map(|i| i.to_string().len()).max().unwrap();

        let headers = (1..=self.cols)
            .map(|c| format!("{c: ^max_len$}"))
            .fold("     ".to_owned(), |acc, s| format!("{acc} | {s}"));
        writeln!(f, "{headers}").unwrap();

        let sep_line = (0..self.cols)
            .map(|_| format!("{:->max_len$}", ""))
            .fold("-----".to_owned(), |acc, s| format!("{acc}-+-{s}"));
        writeln!(f, "{sep_line}-+").unwrap();
        for row in 0..self.rows {
            let col_label = row + 1;
            write!(f, "{col_label: >5}").unwrap();

            let data_line = (0..self.cols)
                .map(|c| &self.data[c + row * self.cols])
                .fold("".to_owned(), |acc, i| format!("{acc} | {i: >max_len$}"));

            writeln!(f, "{data_line}").unwrap();
        }

        Ok(())
    }
}

impl CodeTable {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0; rows * cols],
        }
    }

    fn _next(&self, row: usize, col: usize) -> (usize, usize) {
        if row == 0 {
            (col + 1, 0)
        } else {
            (row - 1, col + 1)
        }
    }

    fn _prev(&self, row: usize, col: usize) -> (usize, usize) {
        if col == 0 {
            (0, row - 1)
        } else {
            (row + 1, col - 1)
        }
    }

    fn get(&self, row: usize, col: usize) -> u128 {
        self.data[(row - 1) * self.cols + (col - 1)]
    }

    fn fill(&mut self, seed: u128) {
        let mut row = 1;
        let mut col = 0;

        self.data[0] = seed;

        while row < self.rows && col < self.cols {
            let prev = self._prev(row, col);
            let prev = self.data[prev.0 * self.cols + prev.1];
            // self.data[row * self.cols + col] = prev + 1;
            self.data[row * self.cols + col] = (prev * 252533_u128) % 33554393_u128;
            (row, col) = self._next(row, col);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeTable;

    #[test]
    fn test_examples() {
        let mut t = CodeTable::new(11, 11);
        t.fill(20151125);

        println!("{t}");

        assert_eq!(t.get(1, 1), 20151125);
        assert_eq!(t.get(1, 2), 18749137);
        assert_eq!(t.get(1, 3), 17289845);
        assert_eq!(t.get(1, 4), 30943339);
        assert_eq!(t.get(1, 5), 10071777);
        assert_eq!(t.get(1, 6), 33511524);

        assert_eq!(t.get(2, 1), 31916031);
        assert_eq!(t.get(2, 2), 21629792);
        assert_eq!(t.get(2, 3), 16929656);
        assert_eq!(t.get(2, 4), 7726640);
        assert_eq!(t.get(2, 5), 15514188);
        assert_eq!(t.get(2, 6), 4041754);

        assert_eq!(t.get(3, 1), 16080970);
        assert_eq!(t.get(3, 2), 8057251);
        assert_eq!(t.get(3, 3), 1601130);
        assert_eq!(t.get(3, 4), 7981243);
        assert_eq!(t.get(3, 5), 11661866);
        assert_eq!(t.get(3, 6), 16474243);

        assert_eq!(t.get(4, 1), 24592653);
        assert_eq!(t.get(4, 2), 32451966);
        assert_eq!(t.get(4, 3), 21345942);
        assert_eq!(t.get(4, 4), 9380097);
        assert_eq!(t.get(4, 5), 10600672);
        assert_eq!(t.get(4, 6), 31527494);

        assert_eq!(t.get(5, 1), 77061);
        assert_eq!(t.get(5, 2), 17552253);
        assert_eq!(t.get(5, 3), 28094349);
        assert_eq!(t.get(5, 4), 6899651);
        assert_eq!(t.get(5, 5), 9250759);
        assert_eq!(t.get(5, 6), 31663883);

        assert_eq!(t.get(6, 1), 33071741);
        assert_eq!(t.get(6, 2), 6796745);
        assert_eq!(t.get(6, 3), 25397450);
        assert_eq!(t.get(6, 4), 24659492);
        assert_eq!(t.get(6, 5), 1534922);
        assert_eq!(t.get(6, 6), 27995004);
    }

    #[test]
    fn test_input() {
        // To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075.
        let row = 2981;
        let col = 3075;

        let size = row + col - 1;

        let mut t = CodeTable::new(size, size);
        t.fill(20151125);

        assert_eq!(t.get(row, col), 9132360);
    }
}
