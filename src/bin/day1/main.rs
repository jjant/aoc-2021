fn main() {
    let input_file = include_str!("input.txt");

    let measurements = input_file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut result = 0;
    for following_measurements in measurements.windows(2) {
        let today = following_measurements[0];
        let tomorrow = following_measurements[1];

        result += (today < tomorrow) as i32;
    }

    println!("{:?}", result);
}
