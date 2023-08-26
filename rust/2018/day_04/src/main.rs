#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display, ops::Range};

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Advent of Code 2018 - day 04");
}

lazy_static! {
    static ref RE_LOG_LINE: Regex = Regex::new(r"^\[([^\]]+)\] (.*)$").unwrap();
    static ref RE_DATE: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})$").unwrap();
    static ref RE_BEGIN_SHIFT: Regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl From<&str> for Timestamp {
    fn from(value: &str) -> Self {
        let caps = RE_DATE.captures(value).unwrap();

        Self {
            year: caps[1].parse().unwrap(),
            month: caps[2].parse().unwrap(),
            day: caps[3].parse().unwrap(),
            hour: caps[4].parse().unwrap(),
            minute: caps[5].parse().unwrap(),
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Guard {
    id: u16,
}

impl From<&str> for Guard {
    fn from(value: &str) -> Self {
        Self {
            id: value.parse().unwrap(),
        }
    }
}

impl Guard {
    fn new(id: u16) -> Self {
        Self { id }
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guard #{}", self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LogEvent {
    BeginShift { guard: Guard },
    FallsAsleep,
    WakesUp,
}

impl From<&str> for LogEvent {
    fn from(value: &str) -> Self {
        match value {
            "falls asleep" => return Self::FallsAsleep,
            "wakes up" => return Self::WakesUp,
            _ => (),
        }

        let caps = RE_BEGIN_SHIFT.captures(value).unwrap();
        Self::BeginShift {
            guard: caps[1].into(),
        }
    }
}

impl Display for LogEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogEvent::BeginShift { guard } => write!(f, "{} begins shift", guard),
            LogEvent::FallsAsleep => write!(f, "falls asleep"),
            LogEvent::WakesUp => write!(f, "wakes up"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LogEntry {
    timestamp: Timestamp,
    event: LogEvent,
}

impl Eq for LogEntry {}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for LogEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<&str> for LogEntry {
    fn from(value: &str) -> Self {
        let caps = RE_LOG_LINE.captures(value).unwrap();

        Self {
            timestamp: caps[1].into(),
            event: caps[2].into(),
        }
    }
}

impl LogEntry {
    fn from_multiple(logs: &str) -> Vec<Self> {
        let mut logs: Vec<Self> = logs.lines().map(|line| line.into()).collect();
        logs.sort();
        logs
    }
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.timestamp, self.event)
    }
}

fn compute_sleep_ranges(logs: &[LogEntry]) -> HashMap<Guard, Vec<Range<u8>>> {
    let mut sleep_ranges = HashMap::new();

    let mut cur_guard = match logs[0].event {
        LogEvent::BeginShift { guard } => guard,
        _ => panic!("First log entry must be start of a guard shift!"),
    };
    let mut is_asleep = false;
    let mut last_ts = logs[0].timestamp;

    for i in 1..logs.len() {
        let entry = &logs[i];
        let mut record_sleep_time = false;
        match entry.event {
            LogEvent::BeginShift { guard } => {
                record_sleep_time = is_asleep;
                cur_guard = guard;
            }
            LogEvent::FallsAsleep => is_asleep = true,
            LogEvent::WakesUp => record_sleep_time = true,
        }
        if record_sleep_time {
            compute_sleep_range(last_ts, entry.timestamp, &cur_guard, &mut sleep_ranges);
            is_asleep = false;
        }
        last_ts = entry.timestamp;
    }

    sleep_ranges
}

fn compute_sleep_range(
    prev_ts: Timestamp,
    cur_ts: Timestamp,
    guard: &Guard,
    sleep_log: &mut HashMap<Guard, Vec<Range<u8>>>,
) {
    let start = if prev_ts.hour == 0 { prev_ts.minute } else { 0 };
    let end = if cur_ts.minute > prev_ts.minute {
        cur_ts.minute
    } else {
        59
    };
    let range = start..end;

    if let Some(ranges) = sleep_log.get_mut(guard) {
        ranges.push(range);
    } else {
        sleep_log.insert(*guard, vec![range]);
    }
}

fn compute_minutes_asleep(ranges: &[Range<u8>]) -> u32 {
    ranges.iter().map(|r| r.end as u32 - r.start as u32).sum()
}

fn select_most_common_minute(ranges: &[Range<u8>]) -> (u8, usize) {
    ranges
        .iter()
        .map(|r| r.clone().into_iter())
        .flatten()
        .fold(HashMap::new(), |mut acc, m| {
            if let Some(count) = acc.get_mut(&m) {
                *count += 1;
            } else {
                acc.insert(m, 1);
            };
            acc
        })
        .iter()
        .map(|(minute, count)| (*minute, *count))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
}

fn strategy_1(logs: &[LogEntry]) -> (Guard, u8) {
    let sleep_ranges = compute_sleep_ranges(logs);

    let guard_most_asleep = sleep_ranges
        .iter()
        .map(|(g, r)| (g, compute_minutes_asleep(r)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .0;
    let most_common_minute_spent_asleep =
        select_most_common_minute(&sleep_ranges[guard_most_asleep]);

    (*guard_most_asleep, most_common_minute_spent_asleep.0)
}

fn strategy_2(logs: &[LogEntry]) -> (Guard, u8) {
    let sleep_ranges = compute_sleep_ranges(logs);

    let (guard, (minute, _)) = sleep_ranges
        .iter()
        .map(|(guard, ranges)| (guard, select_most_common_minute(ranges)))
        .max_by(|(_, (_, count1)), (_, (_, count2))| count1.cmp(count2))
        .unwrap();

    (*guard, minute)
}

#[cfg(test)]
mod tests {
    use crate::{strategy_1, strategy_2, LogEntry};

    #[test]
    fn test_examples() {
        let logs = "\
            [1518-11-01 00:00] Guard #10 begins shift\n\
            [1518-11-01 00:05] falls asleep\n\
            [1518-11-01 00:25] wakes up\n\
            [1518-11-01 00:30] falls asleep\n\
            [1518-11-01 00:55] wakes up\n\
            [1518-11-01 23:58] Guard #99 begins shift\n\
            [1518-11-02 00:40] falls asleep\n\
            [1518-11-02 00:50] wakes up\n\
            [1518-11-03 00:05] Guard #10 begins shift\n\
            [1518-11-03 00:24] falls asleep\n\
            [1518-11-03 00:29] wakes up\n\
            [1518-11-04 00:02] Guard #99 begins shift\n\
            [1518-11-04 00:36] falls asleep\n\
            [1518-11-04 00:46] wakes up\n\
            [1518-11-05 00:45] falls asleep\n\
            [1518-11-05 00:03] Guard #99 begins shift\n\
            [1518-11-05 00:55] wakes up\
        ";
        let logs = LogEntry::from_multiple(logs);

        let (guard, best_minute) = strategy_1(&logs);
        assert_eq!(guard.id, 10);
        assert_eq!(best_minute, 24);
        assert_eq!(guard.id as u32 * best_minute as u32, 10 * 24);

        let (guard, best_minute) = strategy_2(&logs);
        assert_eq!(guard.id, 99);
        assert_eq!(best_minute, 45);
        assert_eq!(guard.id as u32 * best_minute as u32, 99 * 45);
    }

    #[test]
    fn test_input() {
        let logs = std::fs::read_to_string("input/log.txt").unwrap();
        let logs = LogEntry::from_multiple(&logs);

        let (guard, best_minute) = strategy_1(&logs);
        assert_eq!(guard.id, 2663);
        assert_eq!(best_minute, 38);
        assert_eq!(guard.id as u32 * best_minute as u32, 101194);

        let (guard, best_minute) = strategy_2(&logs);
        assert_eq!(guard.id, 2917);
        assert_eq!(best_minute, 35);
        assert_eq!(guard.id as u32 * best_minute as u32, 2917 * 35);
    }
}
