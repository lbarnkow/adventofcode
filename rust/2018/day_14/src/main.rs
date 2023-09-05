#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2018 - day 14");
}

static NUM_ELVES: usize = 2;

fn fill_next(scores: &mut Vec<usize>, current_recipe: &mut [usize]) {
    let next_score: usize = current_recipe.iter().map(|idx| scores[*idx]).sum();
    if next_score < 10 {
        scores.push(next_score);
    } else {
        scores.push(1);
        scores.push(next_score % 10);
    }
    for idx in 0..NUM_ELVES {
        current_recipe[idx] =
            (current_recipe[idx] + scores[current_recipe[idx]] + 1) % scores.len();
    }
}

fn next_scores(initial_scores: &[usize], num_rounds: usize, num_next_scores: usize) -> String {
    let mut result = String::with_capacity(num_next_scores);
    let mut scores = Vec::<usize>::with_capacity(num_rounds * 2);
    for score in initial_scores {
        if *score > 9 {
            panic!("Initial score higher that 9!");
        }
        scores.push(*score);
    }
    let mut current_recipe: Vec<usize> = (0..NUM_ELVES).collect();

    while scores.len() < (num_rounds + num_next_scores) {
        fill_next(&mut scores, &mut current_recipe);
    }

    for score in &scores[num_rounds..num_rounds + num_next_scores] {
        result.push(char::from_u32('0' as u32 + *score as u32).unwrap());
    }

    result
}

fn num_recipes_before(initial_scores: &[usize], target: &str) -> usize {
    let target: Vec<usize> = target
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                (c as u32 - '0' as u32) as usize
            } else {
                panic!("Illegal character in target sequence")
            }
        })
        .collect();

    let mut scores = Vec::<usize>::new();
    for score in initial_scores {
        if *score > 9 {
            panic!("Initial score higher that 9!");
        }
        scores.push(*score);
    }
    let mut current_recipe: Vec<usize> = (0..NUM_ELVES).collect();

    loop {
        let next_score: usize = current_recipe.iter().map(|idx| scores[*idx]).sum();
        if next_score < 10 {
            scores.push(next_score);
        } else {
            scores.push(next_score / 10);
            scores.push(next_score % 10);
        }
        for idx in 0..NUM_ELVES {
            current_recipe[idx] =
                (current_recipe[idx] + scores[current_recipe[idx]] + 1) % scores.len();
        }

        if scores.len() > target.len() {
            let idx = scores.len() - target.len();
            if scores[idx..(idx + target.len())] == target {
                return idx;
            } else if scores[(idx - 1)..(idx + target.len() - 1)] == target {
                return idx - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{next_scores, num_recipes_before};

    static INITIAL_SCORES: [usize; 2] = [3, 7];

    #[test]
    fn test_example() {
        let num_rounds = 9;
        assert_eq!(next_scores(&INITIAL_SCORES, num_rounds, 10), "5158916779");

        let num_rounds = 5;
        assert_eq!(next_scores(&INITIAL_SCORES, num_rounds, 10), "0124515891");

        let num_rounds = 18;
        assert_eq!(next_scores(&INITIAL_SCORES, num_rounds, 10), "9251071085");

        let num_rounds = 2018;
        assert_eq!(next_scores(&INITIAL_SCORES, num_rounds, 10), "5941429882");
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(num_recipes_before(&INITIAL_SCORES, "51589"), 9);
        assert_eq!(num_recipes_before(&INITIAL_SCORES, "01245"), 5);
        assert_eq!(num_recipes_before(&INITIAL_SCORES, "92510"), 18);
        assert_eq!(num_recipes_before(&INITIAL_SCORES, "59414"), 2018);
    }

    #[test]
    fn test_input() {
        let num_rounds = 330121;
        assert_eq!(next_scores(&INITIAL_SCORES, num_rounds, 10), "3410710325");
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(num_recipes_before(&INITIAL_SCORES, "330121"), 20216138);
    }
}
