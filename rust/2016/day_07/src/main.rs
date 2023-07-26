#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2016 - day 07");
}

#[derive(Debug, PartialEq)]
enum State {
    SupernetSeq,
    HypernetSeq,
}

fn contains_autonomous_bridge_bypass_annotation(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let mut iter = s.chars();
    let mut c1 = iter.next().unwrap();
    let mut c2 = iter.next().unwrap();
    let mut c3 = iter.next().unwrap();

    while let Some(c) = iter.next() {
        if c1 != c2 && c2 == c3 && c1 == c {
            return true;
        }
        c1 = c2;
        c2 = c3;
        c3 = c;
    }

    false
}

fn find_area_broadcast_accessors(s: &str) -> Vec<&str> {
    if s.len() < 3 {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut iter = s.chars().enumerate();
    let mut c1 = iter.next().unwrap().1;
    let mut c2 = iter.next().unwrap().1;

    while let Some((idx, c)) = iter.next() {
        if c1 != c2 && c1 == c {
            result.push(&s[idx - 2..idx + 1]);
        }
        c1 = c2;
        c2 = c;
    }

    result
}

fn split_ipv7(ip: &str) -> Vec<(&str, State)> {
    let mut start_idx = 0;
    let mut state = State::SupernetSeq;
    let mut result = Vec::new();

    for (idx, c) in ip.chars().enumerate() {
        if c == '[' || c == ']' {
            result.push((&ip[start_idx..idx], state));
            state = if c == '[' {
                State::HypernetSeq
            } else {
                State::SupernetSeq
            };
            start_idx = idx + 1;
        }
    }

    result.push((&ip[start_idx..ip.len()], state));
    result
}

fn ipv7_supports_transport_layer_snooping(ip: &str) -> bool {
    let mut num_abba = 0;

    for (segment, state) in split_ipv7(ip) {
        match state {
            State::SupernetSeq => {
                if contains_autonomous_bridge_bypass_annotation(segment) {
                    num_abba += 1;
                }
            }
            State::HypernetSeq => {
                if contains_autonomous_bridge_bypass_annotation(segment) {
                    return false;
                }
            }
        }
    }

    num_abba > 0
}

fn ipv7_supports_super_secret_listening(ip: &str) -> bool {
    let mut abas = Vec::new();
    let mut hypernets = Vec::new();

    for (segment, state) in split_ipv7(ip) {
        match state {
            State::SupernetSeq => abas.append(&mut find_area_broadcast_accessors(segment)),
            State::HypernetSeq => hypernets.push(segment),
        }
    }

    for aba in abas {
        let bab = format!("{}{}", &aba[1..3], &aba[1..2]);
        for hypernet in &hypernets {
            if hypernet.contains(&bab) {
                return true;
            }
        }
    }

    false
}

fn count_ips_supporting_tls(ips: &str) -> usize {
    ips.lines()
        .filter(|ip| ipv7_supports_transport_layer_snooping(ip))
        .count()
}

fn count_ips_supporting_ssl(ips: &str) -> usize {
    ips.lines()
        .filter(|ip| ipv7_supports_super_secret_listening(ip))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{
        contains_autonomous_bridge_bypass_annotation, count_ips_supporting_ssl,
        count_ips_supporting_tls, find_area_broadcast_accessors,
        ipv7_supports_super_secret_listening, ipv7_supports_transport_layer_snooping, split_ipv7,
        State,
    };

    #[test]
    fn test_contains_abba() {
        assert_eq!(contains_autonomous_bridge_bypass_annotation("abba"), true);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("qrst"), false);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("abcd"), false);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("xyyx"), true);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("aaaa"), false);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("tyui"), false);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("ioxxoj"), true);
        assert_eq!(
            contains_autonomous_bridge_bypass_annotation("zxcvbn"),
            false
        );
        assert_eq!(contains_autonomous_bridge_bypass_annotation("mnop"), false);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("bddb"), true);
        assert_eq!(contains_autonomous_bridge_bypass_annotation("qwer"), false);
        assert_eq!(
            contains_autonomous_bridge_bypass_annotation("asdfgh"),
            false
        );
    }

    #[test]
    fn test_split_ipv7() {
        assert_eq!(
            split_ipv7("babzuaikmedruqsuuv[emlhynmvfhsigdryo]iyblsqlpplrlahtwr"),
            vec![
                ("babzuaikmedruqsuuv", State::SupernetSeq),
                ("emlhynmvfhsigdryo", State::HypernetSeq),
                ("iyblsqlpplrlahtwr", State::SupernetSeq),
            ]
        );
        assert_eq!(
            split_ipv7("[abc]def[ghi]"),
            vec![
                ("", State::SupernetSeq),
                ("abc", State::HypernetSeq),
                ("def", State::SupernetSeq),
                ("ghi", State::HypernetSeq),
                ("", State::SupernetSeq),
            ]
        );
    }

    #[test]
    fn test_ipv7_supports_tls() {
        assert_eq!(
            ipv7_supports_transport_layer_snooping("abba[mnop]qrst"),
            true
        );
        assert_eq!(
            ipv7_supports_transport_layer_snooping("abcd[bddb]xyyx"),
            false
        );
        assert_eq!(
            ipv7_supports_transport_layer_snooping("aaaa[qwer]tyui"),
            false
        );
        assert_eq!(
            ipv7_supports_transport_layer_snooping("ioxxoj[asdfgh]zxcvb"),
            true
        );

        assert_eq!(
            ipv7_supports_transport_layer_snooping(
                "babzuaikmedruqsuuv[emlhynmvfhsigdryo]iyblsqlpplrlahtwr"
            ),
            true
        );
    }

    #[test]
    fn test_find_area_broadcast_accessors() {
        assert_eq!(find_area_broadcast_accessors("aba"), vec!["aba"]);
        assert_eq!(find_area_broadcast_accessors("zazbz"), vec!["zaz", "zbz"]);
        assert_eq!(find_area_broadcast_accessors("zazabz"), vec!["zaz", "aza"]);
    }

    #[test]
    fn test_ipv7_supports_ssl() {
        assert_eq!(ipv7_supports_super_secret_listening("aba[bab]xyz"), true);
        assert_eq!(ipv7_supports_super_secret_listening("xyx[xyx]xyx"), false);
        assert_eq!(ipv7_supports_super_secret_listening("aaa[kek]eke"), true);
        assert_eq!(ipv7_supports_super_secret_listening("zazbz[bzb]cdb"), true);
    }

    #[test]
    fn test_example() {
        let ips = "\
            abba[mnop]qrst\n\
            abcd[bddb]xyyx\n\
            aaaa[qwer]tyui\n\
            ioxxoj[asdfgh]zxcvbn\
        ";

        assert_eq!(count_ips_supporting_tls(ips), 2);

        let ips = "\
            aba[bab]xyz\n\
            xyx[xyx]xyx\n\
            aaa[kek]eke\n\
            zazbz[bzb]cdb\
        ";

        assert_eq!(count_ips_supporting_ssl(ips), 3);
    }

    #[test]
    fn test_input() {
        let ips = std::fs::read_to_string("input/ips.txt").unwrap();

        assert_eq!(count_ips_supporting_tls(&ips), 110);

        assert_eq!(count_ips_supporting_ssl(&ips), 242);
    }
}
