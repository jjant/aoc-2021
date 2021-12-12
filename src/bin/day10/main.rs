#[derive(Debug, PartialEq)]
enum CharState {
    Open,
    Close,
}

#[derive(Debug, PartialEq)]
enum CharType {
    Parens,   // ()
    Brackets, // []
    Braces,   // {}
    Angle,    // <>
}

impl CharType {
    fn error_score(&self) -> i64 {
        match self {
            Parens => 3,
            Brackets => 57,
            Braces => 1197,
            Angle => 25137,
        }
    }

    fn completion_score(&self) -> i64 {
        match self {
            Parens => 1,
            Brackets => 2,
            Braces => 3,
            Angle => 4,
        }
    }
}

#[derive(Debug)]
struct Char {
    state: CharState,
    char_type: CharType,
}

use CharState::*;
use CharType::*;

impl Char {
    fn new(state: CharState, char_type: CharType) -> Self {
        Self { state, char_type }
    }

    fn from_char(c: char) -> Self {
        match c {
            '(' => Self::new(Open, Parens),
            '[' => Self::new(Open, Brackets),
            '{' => Self::new(Open, Braces),
            '<' => Self::new(Open, Angle),
            ')' => Self::new(Close, Parens),
            ']' => Self::new(Close, Brackets),
            '}' => Self::new(Close, Braces),
            '>' => Self::new(Close, Angle),
            _ => panic!("Invalid char: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self {
                state: Open,
                char_type: Parens,
            } => '(',
            Self {
                state: Open,
                char_type: Brackets,
            } => '[',
            Self {
                state: Open,
                char_type: Braces,
            } => '{',
            Self {
                state: Open,
                char_type: Angle,
            } => '<',
            Self {
                state: Close,
                char_type: Parens,
            } => ')',
            Self {
                state: Close,
                char_type: Brackets,
            } => ']',
            Self {
                state: Close,
                char_type: Braces,
            } => '}',
            Self {
                state: Close,
                char_type: Angle,
            } => '>',
        }
    }

    #[allow(dead_code)]
    fn to_string(chars: &[Char]) -> String {
        chars.iter().map(Char::to_char).collect()
    }
}

fn parse_line(line: &str) -> Result<Vec<Char>, CharType> {
    let mut res = vec![];
    for c in line.chars() {
        let char = Char::from_char(c);

        match char.state {
            Open => {
                res.push(char);
            }
            Close => {
                let last = res.last().unwrap();
                let is_bad = last.state == Open && last.char_type != char.char_type;

                if is_bad {
                    return Err(char.char_type);
                } else {
                    // If we are good, we closed the thing so we can pop the opening one.
                    res.pop();
                }
            }
        }
    }

    Ok(res)
}

fn part1(lines: &[&str]) -> i64 {
    let mut res = 0;

    for line in lines.iter() {
        if let Err(char_type) = parse_line(line) {
            res += char_type.error_score()
        }
    }

    res
}

fn part2(lines: &[&str]) -> i64 {
    let mut scores = lines
        .iter()
        .filter_map(|line| {
            if let Ok(remaining) = parse_line(line) {
                let completion_score = remaining
                    .iter()
                    .rev()
                    .map(|c| c.char_type.completion_score())
                    .fold(0, |acc, elem| acc * 5 + elem);

                Some(completion_score)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let lines = input_file.lines().collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&lines));
    println!("Part 2:");
    println!("\t{}", part2(&lines));
}
