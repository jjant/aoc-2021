use std::cmp::Ordering;

const DIGITS: usize = 12;

fn count_most_popular_digit(numbers: &[u32], digit_index: usize) -> i32 {
    let mut result = 0;

    for number in numbers.iter() {
        let mask = 1 << digit_index;
        let digit_is_one = number & mask != 0;

        if digit_is_one {
            result += 1
        } else {
            result -= 1
        }
    }

    result
}

/// It keeps that element regardless of whether it passes the predicate or not.
fn retain_if_many<T, F>(vec: &mut Vec<T>, filter: F)
where
    F: Fn(&T) -> bool,
{
    let mut i = 0;
    while i < vec.len() && vec.len() > 1 {
        if !filter(&mut vec[i]) {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Indices in the context of binary positions usually start at 0 on the right side:
///
///   0b0101010
///           ^
///           |
///  LH Index = 0
///
/// However, this exercise counts indices as starting from the left hand side:
///
///   0b0101010
///     ^
///     |
/// RH Index = 0
///
/// This function takes a LH index and converts into a RH index.
fn right_hand_index(left_hand_index: usize) -> usize {
    DIGITS - left_hand_index - 1
}

fn part1(numbers: &[u32]) -> u32 {
    let digit_counts: Vec<i32> = (0..DIGITS)
        .map(|index| count_most_popular_digit(numbers, right_hand_index(index)))
        .collect();

    let mut gamma = 0;

    for digit_count in &digit_counts {
        gamma <<= 1;
        gamma |= match digit_count.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => panic!("Invalid input, equal number of ones and zeroes."),
            Ordering::Less => 0,
        }
    }
    let epsilon = !gamma & 0b1111_1111_1111;

    gamma * epsilon
}

/// Filters things out (based on the predicate) until there's only one left.

fn part2(numbers: &[u32]) -> u32 {
    // oxygen generator rating
    //  pick most common digit
    //  on conflict: 1
    // CO2 scrubber rating
    //  pick least common digit
    //  on conflict: 0
    let mut oxygen_candidates = numbers.to_owned();
    let mut co2_candidates = numbers.to_owned();

    for digit_index in 0..DIGITS {
        let actual_position = right_hand_index(digit_index);

        let oxygen_digit_count = count_most_popular_digit(&oxygen_candidates, actual_position);
        match oxygen_digit_count.cmp(&0) {
            Ordering::Greater => {
                // 1 was the most common digit
                retain_if_many(&mut oxygen_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) != 0
                });
            }
            Ordering::Equal => {
                retain_if_many(&mut oxygen_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) != 0
                });
            }
            Ordering::Less => {
                // 0 was the most common digit
                retain_if_many(&mut oxygen_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) == 0
                });
            }
        };

        let co2_digit_count = count_most_popular_digit(&co2_candidates, actual_position);
        match co2_digit_count.cmp(&0) {
            Ordering::Greater => {
                // 1 was the most common digit
                retain_if_many(&mut co2_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) == 0
                });
            }
            Ordering::Equal => {
                retain_if_many(&mut co2_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) == 0
                });
            }
            Ordering::Less => {
                // 0 was the most common digit
                retain_if_many(&mut co2_candidates, |num| {
                    let mask = 1 << actual_position;

                    (*num & mask) != 0
                });
            }
        };
    }
    let oxygen_generator_rating = oxygen_candidates[0];
    let co2_scrubber_rating = co2_candidates[0];

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let numbers: Vec<u32> = input_file
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&numbers));
    println!("Part 2:");
    println!("\t{}", part2(&numbers));
}
