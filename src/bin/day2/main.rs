use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}
use Instruction::*;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction_name, value) = s.split_once(' ').unwrap();
        let value = value.parse().unwrap();

        let instruction = match instruction_name {
            "forward" => Forward(value),
            "down" => Down(value),
            "up" => Up(value),
            _ => panic!("Wrong instruction provided {}", instruction_name),
        };

        Ok(instruction)
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        match instruction {
            Forward(dx) => x += dx,
            Down(dy) => y += dy,
            Up(minus_dy) => y -= minus_dy,
        }
    }

    x * y
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for instruction in instructions {
        match instruction {
            Forward(value) => {
                x += value;
                y += aim * value;
            }
            Down(value) => aim += value,
            Up(value) => aim -= value,
        }
    }

    x * y
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let instructions = input_file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Instruction>>();

    println!("Part 1:");
    println!("\t{}", part1(&instructions));
    println!("Part 2:");
    println!("\t{}", part2(&instructions));
}
