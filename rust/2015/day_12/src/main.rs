#![allow(dead_code)]
use serde_json::{Map, Number, Value};

fn main() {
    println!("Advent of Code 2015 - day 12");
}

fn sum_from_array(arr: &Vec<Value>) -> i64 {
    arr.iter().map(|v| sum_from_value(v)).sum()
}

fn sum_from_object(o: &Map<String, Value>) -> i64 {
    o.iter().map(|(_, v)| sum_from_value(v)).sum()
}

fn parse_number(n: &Number) -> i64 {
    if n.is_i64() {
        n.as_i64().unwrap()
    } else {
        panic!("Only integers are supported!")
    }
}

fn sum_from_value(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => parse_number(n),
        Value::String(_) => 0,
        Value::Array(arr) => sum_from_array(&arr),
        Value::Object(o) => sum_from_object(o),
    }
}

fn sum_numbers(json: &str) -> i64 {
    let json: Value = serde_json::from_str(json).unwrap();
    sum_from_value(&json)
}

fn sum_from_array_red(arr: &Vec<Value>) -> i64 {
    arr.iter().map(|v| sum_from_value_red(v)).sum()
}

fn sum_from_object_red(o: &Map<String, Value>) -> i64 {
    let count_red = o
        .iter()
        .filter(|(_, v)| v.is_string())
        .filter(|(_, v)| v.as_str().unwrap() == "red")
        .count();

    if count_red == 0 {
        o.iter().map(|(_, v)| sum_from_value_red(v)).sum()
    } else {
        0
    }
}

fn sum_from_value_red(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => parse_number(n),
        Value::String(_) => 0,
        Value::Array(arr) => sum_from_array_red(&arr),
        Value::Object(o) => sum_from_object_red(o),
    }
}

fn sum_numbers_red(json: &str) -> i64 {
    let json: Value = serde_json::from_str(json).unwrap();
    sum_from_value_red(&json)
}

#[cfg(test)]
mod tests {
    use crate::{sum_numbers, sum_numbers_red};

    #[test]
    fn test_examples() {
        assert_eq!(sum_numbers(r#""bla""#), 0);
        assert_eq!(sum_numbers(r#"123"#), 123);
        assert_eq!(sum_numbers(r#"[]"#), 0);

        assert_eq!(sum_numbers_red(r#""bla""#), 0);
        assert_eq!(sum_numbers_red(r#"123"#), 123);
        assert_eq!(sum_numbers_red(r#"[]"#), 0);

        assert_eq!(sum_numbers_red(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(sum_numbers_red(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(sum_numbers_red(r#"[1,"red",5]"#), 6);
    }

    #[test]
    fn test_input() {
        let json = std::fs::read_to_string("input/doc.json").unwrap();
        assert_eq!(sum_numbers(&json), 191164);
        assert_eq!(sum_numbers_red(&json), 87842);
    }
}
