fn solve(numbers: &[usize], days_to_run: usize) -> usize {
    let mut fish_count = [0; 9];

    for n in numbers {
        fish_count[*n] += 1;
    }

    for _ in 0..days_to_run {
        // Simulating 1 day
        let mut zero_fish = 0;
        for i in 0..9 {
            if i == 0 {
                zero_fish = fish_count[0];
            } else {
                fish_count[i - 1] = fish_count[i]
            }
        }
        fish_count[6] += zero_fish;
        fish_count[8] = zero_fish;
    }

    fish_count.iter().sum()
}

fn part1(numbers: &[usize]) -> usize {
    solve(numbers, 80)
}

fn part2(numbers: &[usize]) -> usize {
    solve(numbers, 256)
}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let numbers = input_file
        .split(',')
        .map(|char| char.parse().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1:");
    println!("\t{}", part1(&numbers));
    println!("Part 2:");
    println!("\t{}", part2(&numbers));
}
