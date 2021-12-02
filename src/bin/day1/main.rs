fn count_contiguous_increments(sequence: &[usize]) -> usize {
    sequence.windows(2).fold(0, |result, windows| {
        result + (windows[0] < windows[1]) as usize
    })
}

fn part1(measurements: &[usize]) -> usize {
    count_contiguous_increments(&measurements)
}

fn part2(measurements: &[usize]) -> usize {
    let summed_windows = measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<usize>>();

    count_contiguous_increments(&summed_windows)
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let measurements = input_file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1:");
    println!("\t{}", part1(&measurements));
    println!("Part 2:");
    println!("\t{}", part2(&measurements));
}
