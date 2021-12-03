use std::cmp::Ordering;

const DIGITS: usize = 12;

fn part1(lines: &[&str]) -> u32 {
    let mut digit_count: [i32; DIGITS] = [0; DIGITS];

    for line in lines {
        for (index, c) in line.chars().enumerate() {
            digit_count[index] += if c == '1' { 1 } else { -1 }
        }
    }

    let mut gamma = 0;

    for digit in &digit_count {
        gamma <<= 1;
        gamma |= match digit.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => panic!("Invalid input, equal number of ones and zeroes."),
            Ordering::Less => 0,
        }
    }
    let epsilon = !gamma & 0b1111_1111_1111;

    gamma * epsilon
}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let lines = input_file.lines().collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&lines));
    println!("Part 2:");
    // println!("\t{}", part2(&instructions));
}
