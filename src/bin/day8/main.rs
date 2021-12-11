use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Signal {
    id: u32,
}

impl Signal {
    pub fn from_str(signal: &str) -> Self {
        let mut res = 0;
        for char in signal.chars() {
            res = res | 1 << (char as u8 - 'a' as u8)
        }

        Self { id: res }
    }

    pub fn count(&self) -> u32 {
        self.id.count_ones()
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            id: self.id & (!other.id),
        }
    }
}

#[derive(Debug)]
struct SignalDecoder {
    unknown_digits: Vec<Signal>,
    known_digits: HashMap<u32, Signal>,
    reverse_lookup: HashMap<Signal, u32>,
}

impl SignalDecoder {
    pub fn from_line(line: &str) -> Self {
        let unknown_digits: Vec<Signal> = line
            .split_ascii_whitespace()
            .map(|str| Signal::from_str(str))
            .collect();

        let mut decoder = Self {
            unknown_digits,
            known_digits: HashMap::new(),
            reverse_lookup: HashMap::new(),
        };

        decoder.init();

        decoder
    }

    fn init(&mut self) {
        self.find_1_4_7_8();
        self.find_3();
        self.find_9();
        self.find_2();
        self.find_5();
        self.find_0();
        self.find_6();
    }

    // These have unique segment counts
    fn find_1_4_7_8(&mut self) {
        let mut i = 0;
        let len_to_number = [(2, 1), (4, 4), (3, 7), (7, 8)];

        while i < self.unknown_digits.len() {
            let mut inc = true;
            for (len, num) in &len_to_number {
                if self.unknown_digits[i].count() == *len {
                    let signal = self.unknown_digits.remove(i);
                    self.known_digits.insert(*num, signal);
                    self.reverse_lookup.insert(signal, *num);
                    inc = false;
                    break;
                }
            }
            i += inc as usize;
        }
    }

    // (unknown_digit - 7).count() == 2
    fn find_3(&mut self) {
        let mut i = 0;
        while i < self.unknown_digits.len() {
            if self.unknown_digits[i]
                .difference(&self.known_digits[&7])
                .count()
                == 2
            {
                let signal = self.unknown_digits.remove(i);
                self.known_digits.insert(3, signal);
                self.reverse_lookup.insert(signal, 3);

                return;
            } else {
                i += 1;
            }
        }
    }

    // (4 - unknown_digit).count() == 0
    fn find_9(&mut self) {
        let four: Signal = self.known_digits[&4];

        let mut i = 0;
        while i < self.unknown_digits.len() {
            if four.difference(&self.unknown_digits[i]).count() == 0 {
                let signal = self.unknown_digits.remove(i);
                self.known_digits.insert(9, signal);
                self.reverse_lookup.insert(signal, 9);

                return;
            } else {
                i += 1;
            }
        }
    }

    // (9 - unknown_digit).count() == 2
    fn find_2(&mut self) {
        let nine: Signal = self.known_digits[&9];

        let mut i = 0;
        while i < self.unknown_digits.len() {
            if nine.difference(&self.unknown_digits[i]).count() == 2 {
                let signal = self.unknown_digits.remove(i);
                self.known_digits.insert(2, signal);
                self.reverse_lookup.insert(signal, 2);

                return;
            } else {
                i += 1;
            }
        }
    }

    // (2 - unknown_digit).count() == 2
    fn find_5(&mut self) {
        let two: Signal = self.known_digits[&2];

        let mut i = 0;
        while i < self.unknown_digits.len() {
            if two.difference(&self.unknown_digits[i]).count() == 2 {
                let signal = self.unknown_digits.remove(i);
                self.known_digits.insert(5, signal);
                self.reverse_lookup.insert(signal, 5);

                return;
            } else {
                i += 1;
            }
        }
    }

    // (unknown_digit - 5).count() == 2
    fn find_0(&mut self) {
        let five: Signal = self.known_digits[&5];

        let mut i = 0;
        while i < self.unknown_digits.len() {
            if self.unknown_digits[i].difference(&five).count() == 2 {
                let signal = self.unknown_digits.remove(i);
                self.known_digits.insert(0, signal);
                self.reverse_lookup.insert(signal, 0);

                return;
            } else {
                i += 1;
            }
        }
    }

    // Last one missing
    fn find_6(&mut self) {
        let signal = self.unknown_digits.remove(0);
        self.known_digits.insert(6, signal);
        self.reverse_lookup.insert(signal, 6);
    }

    pub fn decode(&self, signal: &Signal) -> u32 {
        self.reverse_lookup[signal]
    }
}

fn has_unique_digits(signals: &str) -> bool {
    let len = signals.len();

    let is_1 = len == 2;
    let is_4 = len == 4;
    let is_7 = len == 3;
    let is_8 = len == 7;

    is_1 || is_4 || is_7 || is_8
}

fn part1(lines: &[&str]) -> usize {
    lines
        .iter()
        .flat_map(|line| line.split_ascii_whitespace())
        .filter(|signal| has_unique_digits(signal))
        .count()
}

fn part2(lines: &[(&str, &str)]) -> u32 {
    let mut total_res = 0;
    for line in lines {
        let (input, output) = line;
        let signal_decoder = SignalDecoder::from_line(input);

        let mut res = 0;
        for signal in output.split_ascii_whitespace() {
            let signal = Signal::from_str(signal);
            let decoded = signal_decoder.decode(&signal);
            res = res * 10 + decoded;
        }
        total_res += res
    }
    total_res
}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let lines = input_file
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .collect::<Vec<_>>();

    let output_lines = lines.iter().map(|t| t.1).collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&output_lines));
    println!("Part 2:");
    println!("\t{}", part2(&lines));
}
