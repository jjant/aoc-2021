fn part1(numbers: &[i32]) -> i32 {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();

    (min..=max)
        .map(|candidate_y| {
            let res: i32 = numbers.iter().map(|n| (n - candidate_y).abs()).sum();

            res
        })
        .min()
        .unwrap()
}

// fn part2(numbers: &[i32]) -> i32 {
// }

fn main() {
    let input_file: &str = include_str!("input.txt");

    let numbers = input_file
        .split(',')
        .map(|char| char.parse().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&numbers));
    println!("Part 2:");
    // println!("\t{}", part2(&numbers));
}
