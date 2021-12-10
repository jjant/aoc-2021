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

fn main() {
    let input_file: &str = include_str!("input.txt");

    let lines = input_file
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .collect::<Vec<_>>();

    let output_lines = lines.iter().map(|t| t.1).collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&output_lines));
}
