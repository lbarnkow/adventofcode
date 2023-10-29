#![allow(dead_code)]

fn main() {
    println!("Advent of Code 2019 - day 16");
}

struct Pattern {
    base: Vec<isize>,
    round: usize,
    index: usize,
    n: usize,
}

impl Pattern {
    fn new(base: &[isize]) -> Self {
        let base = Vec::from_iter(base.iter().copied());
        let mut p = Self {
            base,
            round: 0,
            index: 0,
            n: 0,
        };
        p.reset();
        p
    }

    fn reset(&mut self) {
        self.round = 0;
        self.index = 1;
        self.n = 0;
    }

    fn next_item(&mut self) -> isize {
        let item = self.base[self.index];
        self.n += 1;
        if self.n > self.round {
            self.n = 0;
            self.index += 1;
            if self.index == self.base.len() {
                self.index = 0;
            }
        }
        item
    }

    fn next_round(&mut self) {
        self.round += 1;
        self.index = 0;
        self.n = 1;
    }
}

fn parse_signal(signal: &str) -> Vec<isize> {
    signal
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

static PATTERN: [isize; 4] = [0, 1, 0, -1];

fn fft_slow(signal: &str, rounds: usize) -> Vec<isize> {
    let mut buf_0 = parse_signal(signal);
    let mut buf_1 = buf_0.clone();
    let mut pattern = Pattern::new(&PATTERN);

    let mut write_to_buf_0 = true;

    for _ in 0..rounds {
        write_to_buf_0 = !write_to_buf_0;
        let (r_buf, w_buf) = match write_to_buf_0 {
            true => (&buf_1, &mut buf_0),
            false => (&buf_0, &mut buf_1),
        };

        for w in w_buf.iter_mut() {
            *w = (r_buf
                .iter()
                .map(|d| *d * pattern.next_item())
                .sum::<isize>()
                .unsigned_abs()
                % 10) as isize;
            pattern.next_round()
        }
        pattern.reset();
    }

    match write_to_buf_0 {
        true => buf_0,
        false => buf_1,
    }
}

fn fft_simplified(signal: &str, rounds: usize, offset: usize) -> Vec<isize> {
    let mut buf = parse_signal(signal);

    for _ in 0..rounds {
        let mut sum = 0;
        for i in (offset..buf.len()).rev() {
            sum += buf[i];
            buf[i] = sum % 10;
        }
    }

    buf
}

fn fft(signal: &str, rounds: usize) -> Vec<isize> {
    let offset = signal[0..7].parse::<usize>().unwrap();
    if offset <= (signal.len() / 2) {
        fft_slow(signal, rounds)
    } else {
        fft_simplified(signal, rounds, offset)
    }
}

fn blow_up_input(signal: &str, n: usize) -> String {
    let mut s = String::with_capacity(signal.len() * n);
    for _ in 0..n {
        s.push_str(signal);
    }
    s
}

fn arr_to_str(arr: &[isize], len: usize) -> String {
    arr[0..len].iter().map(|i| i.to_string()).fold(
        String::with_capacity(arr.len()),
        |mut acc, s| {
            acc.push_str(&s);
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::{arr_to_str, blow_up_input, fft, fft_slow};

    #[test]
    fn test_examples() {
        let signal = "12345678";

        let result = fft_slow(signal, 1);
        assert_eq!(arr_to_str(&result, 8), "48226158");
        let result = fft_slow(signal, 2);
        assert_eq!(arr_to_str(&result, 8), "34040438");
        let result = fft_slow(signal, 3);
        assert_eq!(arr_to_str(&result, 8), "03415518");
        let result = fft_slow(signal, 4);
        assert_eq!(arr_to_str(&result, 8), "01029498");

        let signal = "80871224585914546619083218645595";

        let result = fft_slow(signal, 100);
        assert_eq!(arr_to_str(&result, 8), "24176176");

        let signal = "19617804207202209144916044189917";

        let result = fft_slow(signal, 100);
        assert_eq!(arr_to_str(&result, 8), "73745418");

        let signal = "69317163492948606335995924319873";

        let result = fft_slow(signal, 100);
        assert_eq!(arr_to_str(&result, 8), "52432133");
    }

    #[test]
    fn test_examples_part2() {
        // All given inputs start with an "offset" (i.e. the position of the "message") that firmly sits in the latter
        // half of the signal. The way the fft function is defined base on the fixed pattern [0, 1, 0, -1] means that
        // each digit in the latter half of the signal will be the sum of the digit at the same index and all following
        // digits from the previous round/signal (however truncated to the ones digit).
        // This can be shown more clearly by trying the function with simplyfied input signals like
        // "11111111111111111111111111111111" resulting in "xxxxxxxxxxxxxxxx6543210987654321" after one round.
        let signal = blow_up_input("03036732577212944063491565474664", 10_000);
        let result = fft(&signal, 100);
        let skip = signal[0..7].parse::<usize>().unwrap();
        let msg = &result[skip..skip + 8];
        assert_eq!(arr_to_str(msg, 8), "84462026");

        let signal = blow_up_input("02935109699940807407585447034323", 10_000);
        let result = fft(&signal, 100);
        let skip = signal[0..7].parse::<usize>().unwrap();
        let msg = &result[skip..skip + 8];
        assert_eq!(arr_to_str(msg, 8), "78725270");

        let signal = blow_up_input("03081770884921959731165446850517", 10_000);
        let result = fft(&signal, 100);
        let skip = signal[0..7].parse::<usize>().unwrap();
        let msg = &result[skip..skip + 8];
        assert_eq!(arr_to_str(msg, 8), "53553731");
    }

    #[test]
    fn test_input() {
        let signal = std::fs::read_to_string("input/signal.txt").unwrap();

        let result = fft_slow(signal.as_str(), 100);
        assert_eq!(arr_to_str(&result, 8), "74369033");
    }

    #[test]
    fn test_input_part2() {
        let signal = std::fs::read_to_string("input/signal.txt").unwrap();
        let signal = blow_up_input(signal.as_str(), 10_000);
        let result = fft(&signal, 100);
        let skip = signal[0..7].parse::<usize>().unwrap();
        let msg = &result[skip..skip + 8];
        assert_eq!(arr_to_str(msg, 8), "19903864");
    }
}
