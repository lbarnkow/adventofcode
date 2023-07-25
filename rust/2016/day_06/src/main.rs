#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2016 - day 06");
}

enum Selection {
    MostCommon,
    LeastCommon,
}

fn ec_repitition_code(messages: &str, sel: Selection) -> String {
    let len = messages.lines().next().unwrap().len();
    let mut hist = vec![HashMap::new(); len];

    for line in messages.lines() {
        for (idx, c) in line.chars().enumerate() {
            let map = &mut hist[idx];
            if let Some(count) = map.get(&c) {
                map.insert(c, *count + 1);
            } else {
                map.insert(c, 1);
            }
        }
    }

    hist.iter()
        .map(|map| {
            let mut vec = map
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(char, i32)>>();
            vec.sort_by(|(_, a), (_, b)| a.cmp(b));

            match sel {
                Selection::MostCommon => vec.reverse(),
                Selection::LeastCommon => (),
            };
            vec[0].0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{ec_repitition_code, Selection};

    #[test]
    fn test_example() {
        let messages = "\
            eedadn\n\
            drvtee\n\
            eandsr\n\
            raavrd\n\
            atevrs\n\
            tsrnev\n\
            sdttsa\n\
            rasrtv\n\
            nssdts\n\
            ntnada\n\
            svetve\n\
            tesnvt\n\
            vntsnd\n\
            vrdear\n\
            dvrsen\n\
            enarar\
        ";

        assert_eq!(
            ec_repitition_code(messages, Selection::MostCommon),
            "easter"
        );
        assert_eq!(
            ec_repitition_code(messages, Selection::LeastCommon),
            "advent"
        );
    }

    #[test]
    fn test_input() {
        let messages = std::fs::read_to_string("input/messages.txt").unwrap();

        assert_eq!(
            ec_repitition_code(&messages, Selection::MostCommon),
            "gyvwpxaz"
        );
        assert_eq!(
            ec_repitition_code(&messages, Selection::LeastCommon),
            "jucfoary"
        );
    }
}
