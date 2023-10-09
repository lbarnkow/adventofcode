#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2019 - day 01");
}

struct Module {
    mass: usize,
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        Self {
            mass: value.parse().unwrap(),
        }
    }
}

static MIN_FUEL_THRESHOLD: usize = 2;

fn fuel_required(mass: usize) -> usize {
    let third = mass / 3;
    if third > MIN_FUEL_THRESHOLD {
        third - 2
    } else {
        0
    }
}

fn fuel_required_real(mass: usize) -> usize {
    if mass == 0 {
        return 0;
    }
    let fuel = fuel_required(mass);
    fuel + fuel_required_real(fuel)
}

impl Module {
    fn fuel_required(&self) -> usize {
        fuel_required(self.mass)
    }

    fn fuel_required_real(&self) -> usize {
        fuel_required_real(self.mass)
    }
}

#[cfg(test)]
mod tests {
    use crate::Module;

    #[test]
    fn test_examples() {
        assert_eq!(Module::from("12").fuel_required(), 2);
        assert_eq!(Module::from("14").fuel_required(), 2);
        assert_eq!(Module::from("1969").fuel_required(), 654);
        assert_eq!(Module::from("100756").fuel_required(), 33583);

        let modules = "\
            12\n\
            14\n\
            1969\n\
            100756\
        ";

        let fuel_sum: usize = modules
            .lines()
            .map(|line| Module::from(line))
            .map(|m| m.fuel_required())
            .sum();

        assert_eq!(fuel_sum, 2 + 2 + 654 + 33583);

        let fuel_sum: usize = modules
            .lines()
            .map(|line| Module::from(line))
            .map(|m| m.fuel_required_real())
            .sum();

        assert_eq!(fuel_sum, 2 + 2 + 966 + 50346);
    }

    #[test]
    fn test_input() {
        let modules = std::fs::read_to_string("input/modules.txt").unwrap();

        let fuel_sum: usize = modules
            .lines()
            .map(|line| Module::from(line))
            .map(|m| m.fuel_required())
            .sum();

        assert_eq!(fuel_sum, 3426455);

        let fuel_sum: usize = modules
            .lines()
            .map(|line| Module::from(line))
            .map(|m| m.fuel_required_real())
            .sum();

        assert_eq!(fuel_sum, 5136807);
    }
}
