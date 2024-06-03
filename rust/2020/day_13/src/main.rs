#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    println!("Advent of Code 2020 - day 13");
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bus {
    Active { id: usize },
    Inactive {},
}

impl Bus {
    fn id(&self) -> usize {
        match self {
            Self::Active { id } => *id,
            Self::Inactive {} => panic!("Inactive bus!"),
        }
    }
}

impl Bus {
    const fn is_active(&self) -> bool {
        !matches!(self, Self::Inactive {})
    }

    fn next_departure(&self, earliest_departure: usize) -> usize {
        match self {
            Self::Active { id } => {
                (earliest_departure / id) * id + if earliest_departure % id != 0 { *id } else { 0 }
            }
            Self::Inactive {} => usize::MAX,
        }
    }

    fn departs_at(&self, t: usize) -> bool {
        t % self.id() == 0
    }
}

impl TryFrom<&str> for Bus {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match (value, value.parse::<usize>()) {
            ("x", _) => Ok(Self::Inactive {}),
            (_, Ok(id)) => Ok(Self::Active { id }),
            (_, _) => Err(format!("Not a valid bus id: '{value}'!").into()),
        }
    }
}

struct Schedule {
    earliest_departure: usize,
    busses: Vec<Bus>,
}

impl TryFrom<&str> for Schedule {
    type Error = TryFromError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let e: TryFromError = "Bus schedule must have exactly two lines!".into();
        let earliest_departure = lines.next().ok_or_else(|| e.clone())?;
        let busses_raw = lines.next().ok_or_else(|| e.clone())?;
        lines.next().map_or(Ok(()), |_| Err(e))?;

        let earliest_departure = earliest_departure.parse::<usize>().map_err(|_| {
            TryFromError::from("First input line does not contain a valid timestamp!")
        })?;

        let mut busses = Vec::new();
        for bus in busses_raw.split(',') {
            busses.push(bus.try_into()?);
        }

        Ok(Self {
            earliest_departure,
            busses,
        })
    }
}

impl Schedule {
    fn wait_for_next_bus(&self) -> (usize, Bus) {
        let mut best_departure = usize::MAX;
        let mut best_bus = None;

        for bus in &self.busses {
            let next_departure = bus.next_departure(self.earliest_departure);
            if next_departure < best_departure {
                best_departure = next_departure;
                best_bus = Some(bus);
            }
        }

        (best_departure - self.earliest_departure, *best_bus.unwrap())
    }

    fn perfect_staggered_departure(&self) -> usize {
        let mut t = 1;
        let mut step_size = 1;
        for (idx, bus) in self
            .busses
            .iter()
            .enumerate()
            .filter(|(_, b)| b.is_active())
        {
            loop {
                if bus.departs_at(t + idx) {
                    step_size *= bus.id();
                    break;
                } else {
                    t += step_size;
                }
            }
        }

        t
    }
}

#[cfg(test)]
mod tests {
    use crate::{Schedule, TryFromError};

    #[test]
    fn test_examples() -> Result<(), TryFromError> {
        let schedule = "\
            939\n\
            7,13,x,x,59,x,31,19\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let (wait_time, bus) = schedule.wait_for_next_bus();
        assert_eq!(wait_time, 5);
        assert_eq!(bus.id(), 59);
        assert_eq!(wait_time * bus.id(), 295);

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 1_068_781);

        let schedule = "\
            0\n\
            17,x,13,19\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 3417);

        let schedule = "\
            0\n\
            67,7,59,61\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 754018);

        let schedule = "\
            0\n\
            67,x,7,59,61\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 779210);

        let schedule = "\
            0\n\
            67,7,x,59,61\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 1261476);

        let schedule = "\
            0\n\
            1789,37,47,1889\
        ";
        let schedule = Schedule::try_from(schedule)?;

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 1_202_161_486);

        Ok(())
    }

    #[test]
    fn test_input() -> Result<(), TryFromError> {
        let schedule = std::fs::read_to_string("input/busplan.txt").unwrap();
        let schedule = Schedule::try_from(schedule.as_str())?;

        let (wait_time, bus) = schedule.wait_for_next_bus();
        assert_eq!(wait_time, 5);
        assert_eq!(bus.id(), 643);
        assert_eq!(wait_time * bus.id(), 3215);

        let t = schedule.perfect_staggered_departure();
        assert_eq!(t, 1_001_569_619_313_439);

        Ok(())
    }
}
