#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2015 - day 10");
}

fn look_and_say(input: &str, rounds: usize) -> String {
    let mut input = input.to_owned();

    for _ in 0..rounds {
        let mut prev = input.chars().next().unwrap();
        let mut i = 0;
        let mut output = String::with_capacity(input.len());

        for c in input.chars() {
            if c == prev {
                i += 1;
            } else {
                output.push_str(&i.to_string());
                output.push(prev);
                i = 1;
                prev = c;
            }
        }
        output.push_str(&i.to_string());
        output.push(prev);

        input = output;
    }

    input
}

#[cfg(test)]
mod tests {
    use crate::look_and_say;

    #[test]
    fn test_examples() {
        let r = look_and_say("1", 1);
        assert_eq!(&r, "11");
        assert_eq!(r.len(), 2);

        let r = look_and_say("11", 1);
        assert_eq!(&r, "21");
        assert_eq!(r.len(), 2);

        let r = look_and_say("21", 1);
        assert_eq!(&r, "1211");
        assert_eq!(r.len(), 4);

        let r = look_and_say("1211", 1);
        assert_eq!(&r, "111221");
        assert_eq!(r.len(), 6);

        let r = look_and_say("111221", 1);
        assert_eq!(&r, "312211");
        assert_eq!(r.len(), 6);
    }

    #[test]
    fn test_input() {
        assert_eq!(look_and_say("3113322113", 40).len(), 329356);
        assert_eq!(look_and_say("3113322113", 50).len(), 4666278);
    }
}
