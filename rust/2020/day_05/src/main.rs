#![allow(dead_code)]

use std::{fmt::Display, num::ParseIntError};

fn main() {
    println!("Advent of Code 2020 - day 05");
}

#[derive(Debug)]
struct TryFromError {
    msg: String,
}

impl From<&str> for TryFromError {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl From<String> for TryFromError {
    fn from(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for TryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERR: {}", &self.msg)
    }
}

impl From<ParseIntError> for TryFromError {
    fn from(value: ParseIntError) -> Self {
        value.to_string().into()
    }
}

struct BoardingPass {
    code: String,
    row: usize,
    column: usize,
    seat_id: usize,
}

impl TryFrom<&str> for BoardingPass {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 10 {
            return Err(format!(
                "Wrong boarding pass code length! Expected: 10, found: {}!",
                value.len()
            )
            .into());
        }

        let row = value[..7].replace('F', "0").replace('B', "1");
        let column = value[7..].replace('L', "0").replace('R', "1");

        let row = usize::from_str_radix(&row, 2)?;
        let column = usize::from_str_radix(&column, 2)?;

        let seat_id = row * 8 + column;

        Ok(Self {
            code: value.to_owned(),
            row,
            column,
            seat_id,
        })
    }
}

fn find_single_hole(sorted_seat_ids: &[usize]) -> usize {
    let mut prev = *sorted_seat_ids.first().unwrap();

    for seat_id in &sorted_seat_ids[1..] {
        if prev + 2 == *seat_id {
            return prev + 1;
        }
        prev = *seat_id;
    }

    panic!("No hole found!");
}

#[cfg(test)]
mod tests {
    use crate::{find_single_hole, BoardingPass, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let pass = BoardingPass::try_from("FBFBBFFRLR")?;

        assert_eq!(pass.code, "FBFBBFFRLR");
        assert_eq!(pass.row, 44);
        assert_eq!(pass.column, 5);
        assert_eq!(pass.seat_id, 357);

        let pass = BoardingPass::try_from("BFFFBBFRRR")?;

        assert_eq!(pass.code, "BFFFBBFRRR");
        assert_eq!(pass.row, 70);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id, 567);

        let pass = BoardingPass::try_from("FFFBBBFRRR")?;

        assert_eq!(pass.code, "FFFBBBFRRR");
        assert_eq!(pass.row, 14);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id, 119);

        let pass = BoardingPass::try_from("BBFFBBFRLL")?;

        assert_eq!(pass.code, "BBFFBBFRLL");
        assert_eq!(pass.row, 102);
        assert_eq!(pass.column, 4);
        assert_eq!(pass.seat_id, 820);

        Ok(())
    }

    #[test]
    fn test_input() {
        let mut taken_seat_ids = std::fs::read_to_string("input/boardingpasses.txt")
            .unwrap()
            .lines()
            .filter_map(|line| BoardingPass::try_from(line).ok())
            .map(|pass| pass.seat_id)
            .collect::<Vec<_>>();
        taken_seat_ids.sort();

        let max_seat_id = *taken_seat_ids.last().unwrap();
        assert_eq!(max_seat_id, 838);

        let my_seat_id = find_single_hole(&taken_seat_ids);
        assert_eq!(my_seat_id, 714);
    }
}
