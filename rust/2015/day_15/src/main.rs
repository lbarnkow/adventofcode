#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2015 - day 15");
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^(\w+): capacity (-?\d)+, durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$").unwrap();
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn parse(line: &str) -> Self {
        let caps = REGEX.captures(line).unwrap();

        Self {
            name: caps[1].to_owned(),
            capacity: caps[2].parse().unwrap(),
            durability: caps[3].parse().unwrap(),
            flavor: caps[4].parse().unwrap(),
            texture: caps[5].parse().unwrap(),
            calories: caps[6].parse().unwrap(),
        }
    }

    fn parse_multi(ingredients: &str) -> Vec<Self> {
        let mut result = Vec::new();

        for line in ingredients.lines() {
            result.push(Self::parse(line));
        }

        result
    }
}

fn dot_rec(
    permutations: &mut Vec<Vec<i64>>,
    weights: &mut Vec<i64>,
    max_teaspoons: i64,
    remaining_components: u64,
) {
    let current_teaspoons: i64 = weights.iter().sum();

    if remaining_components == 0 {
        if current_teaspoons == max_teaspoons {
            permutations.push(weights.clone());
        }
        return;
    }

    let remaining_teaspoons = max_teaspoons - current_teaspoons;

    for i in 0..=remaining_teaspoons {
        weights.push(i);
        dot_rec(
            permutations,
            weights,
            max_teaspoons,
            remaining_components - 1,
        );
        weights.pop();
    }
}

fn dot(sum: i64, components: u64) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    dot_rec(&mut result, &mut Vec::new(), sum, components);
    result
}

fn calc_recipe_score(recipe: &Vec<i64>, ingredients: &Vec<Ingredient>, calorie_limit: i64) -> i64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (idx, amount) in recipe.iter().enumerate() {
        capacity += *amount * ingredients.get(idx).unwrap().capacity;
        durability += *amount * ingredients.get(idx).unwrap().durability;
        flavor += *amount * ingredients.get(idx).unwrap().flavor;
        texture += *amount * ingredients.get(idx).unwrap().texture;
        calories += *amount * ingredients.get(idx).unwrap().calories;
    }

    if capacity < 0
        || durability < 0
        || flavor < 0
        || texture < 0
        || (calorie_limit > 0 && calories != calorie_limit)
    {
        0
    } else {
        capacity * durability * flavor * texture
    }
}

fn best_scoring_recipe(ingredients: &str, teaspoons: u64, calorie_limit: u64) -> i64 {
    let teaspoons: i64 = teaspoons.try_into().unwrap();
    let calorie_limit: i64 = calorie_limit.try_into().unwrap();
    let ingredients = Ingredient::parse_multi(ingredients);
    let recipes = dot(teaspoons, ingredients.len().try_into().unwrap());

    let mut best_score = i64::MIN;

    for recipe in recipes {
        let score = calc_recipe_score(&recipe, &ingredients, calorie_limit);
        if score > best_score {
            best_score = score;
        }
    }

    best_score
}

#[cfg(test)]
mod tests {
    use crate::best_scoring_recipe;

    #[test]
    fn test_examples() {
        let ingredients = "\
            Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
            Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3\
        ";
        assert_eq!(best_scoring_recipe(ingredients, 100, 0), 62842880);
        assert_eq!(best_scoring_recipe(ingredients, 100, 500), 57600000);
    }

    #[test]
    fn test_input() {
        let ingredients = std::fs::read_to_string("input/ingredients.txt").unwrap();
        assert_eq!(best_scoring_recipe(&ingredients, 100, 0), 13882464);
        assert_eq!(best_scoring_recipe(&ingredients, 100, 500), 11171160);
    }
}
