#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 02");
}

fn build_keypad(raw: &str, rows: usize, cols: usize) -> Vec<Vec<Option<char>>> {
    let mut result: Vec<Vec<Option<char>>> = vec![vec![None; cols + 2]; rows + 2];

    assert_eq!(raw.lines().count(), rows);

    for (row, line) in raw.lines().enumerate() {
        assert_eq!(line.len(), cols);

        for (col, c) in line.chars().enumerate() {
            let c = if c.is_ascii_alphanumeric() {
                Some(c)
            } else if c == '-' {
                None
            } else {
                panic!("Illegal character!")
            };
            result[row + 1][col + 1] = c;
        }
    }

    result
}

fn find_btn(keypad: &Vec<Vec<Option<char>>>, label: char) -> (usize, usize) {
    for (row_idx, row) in keypad.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if let Some(btn) = col {
                if *btn == label {
                    return (row_idx, col_idx);
                }
            }
        }
    }

    panic!("Button label not found!")
}

fn move_pos_2(
    (row, col): (usize, usize),
    dir: char,
    keypad: &Vec<Vec<Option<char>>>,
) -> (usize, usize) {
    let mut new_row = row;
    let mut new_col = col;

    match dir {
        'U' => new_row -= 1,
        'D' => new_row += 1,
        'L' => new_col -= 1,
        'R' => new_col += 1,
        _ => panic!("Illegal step!"),
    }

    if keypad[new_row][new_col].is_some() {
        (new_row, new_col)
    } else {
        (row, col)
    }
}

fn btn_label((row, col): (usize, usize), keypad: &Vec<Vec<Option<char>>>) -> char {
    keypad[row][col].unwrap()
}

fn compute_door_code_2(
    instructions: &str,
    keypad: &Vec<Vec<Option<char>>>,
    initial_btn_label: char,
) -> String {
    let mut btn = find_btn(keypad, initial_btn_label);
    let mut result = String::new();

    for instruction in instructions.lines() {
        for step in instruction.chars() {
            btn = move_pos_2(btn, step, keypad);
        }
        result.push(btn_label(btn, keypad));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{build_keypad, compute_door_code_2};

    #[test]
    fn test_examples() {
        let instructions = "\
            ULL\n\
            RRDDD\n\
            LURDL\n\
            UUUUD\
        ";

        let keypad = "\
            123\n\
            456\n\
            789\
        ";
        let keypad = build_keypad(keypad, 3, 3);

        assert_eq!(&compute_door_code_2(instructions, &keypad, '5'), "1985");
    }

    #[test]
    fn test_input() {
        let instructions = std::fs::read_to_string("input/instructions.txt").unwrap();

        let keypad = "\
            123\n\
            456\n\
            789\
        ";
        let keypad = build_keypad(keypad, 3, 3);

        assert_eq!(&compute_door_code_2(&instructions, &keypad, '5'), "76792");

        let keypad = "\
            --1--\n\
            -234-\n\
            56789\n\
            -ABC-\n\
            --D--\
        ";
        let keypad = build_keypad(keypad, 5, 5);

        assert_eq!(&compute_door_code_2(&instructions, &keypad, '5'), "A7AC3");
    }
}
