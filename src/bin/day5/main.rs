use std::{cmp::Ordering, vec};

struct World(Vec<Vec<i32>>);

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self(vec![vec![0; width]; height])
    }

    pub fn mark(&mut self, point: &Point) {
        self.0[point.y as usize][point.x as usize] += 1;
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for row in self.0.iter() {
            for cell in row.iter() {
                let char = if *cell == 0 {
                    ".".to_owned()
                } else {
                    cell.to_string()
                };
                result += &char;
            }
            result += "\n";
        }

        result
    }
}

fn solve(lines: &[Line], include_diagonals: bool) -> i32 {
    let width = lines
        .iter()
        .map(|line| line.start.x.max(line.end.x))
        .max()
        .unwrap_or(0) as usize
        + 1;
    let height = lines
        .iter()
        .map(|line| line.start.y.max(line.end.y))
        .max()
        .unwrap_or(0) as usize
        + 1;

    let mut world = World::new(width, height);

    for line in lines {
        for point in line.iter(include_diagonals) {
            world.mark(&point);
        }
    }

    let mut overlap_count = 0;
    for row in world.0.iter() {
        for cell in row.iter() {
            if *cell >= 2 {
                overlap_count += 1
            }
        }
    }

    overlap_count
}

fn part1(lines: &[Line]) -> i32 {
    solve(lines, false)
}

fn part2(lines: &[Line]) -> i32 {
    solve(lines, true)
}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let parsed_input = input_file
        .split('\n')
        .map(|line| Line::parse(line))
        .collect::<Option<Vec<_>>>();

    let lines = parsed_input.unwrap();

    println!("Part 1:");
    println!("\t{}", part1(&lines));
    println!("Part 2:");
    println!("\t{}", part2(&lines));
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Parses a string like:
    ///  "0,9"
    pub fn parse(input: &str) -> Option<Self> {
        let (str_x, str_y) = input.split_once(',')?;

        let x = str_x.parse().ok()?;
        let y = str_y.parse().ok()?;

        Some(Self { x, y })
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        }
    }

    pub fn sort2(a: Point, b: Point) -> (Point, Point) {
        match a.cmp(&b) {
            Ordering::Less => (a, b),
            Ordering::Equal => (a, b),
            Ordering::Greater => (b, a),
        }
    }
}

/// Lines assume sorted points.
#[derive(Debug)]
struct Line {
    /// "Top-left" point
    start: Point,
    /// "Bottom-right" point
    end: Point,
}

impl Line {
    /// Parses a string like:
    ///  "0,9 -> 5,9"
    pub fn parse(input: &str) -> Option<Self> {
        let (str_p1, str_p2) = input.split_once(" -> ")?;

        let p1 = Point::parse(str_p1)?;
        let p2 = Point::parse(str_p2)?;

        let (start, end) = Point::sort2(p1, p2);

        Some(Self { start, end })
    }

    pub fn iter(&self, include_diagonals: bool) -> impl Iterator<Item = Point> {
        let is_vertical = self.start.x == self.end.x;
        let is_horizontal = self.start.y == self.end.y;
        let mut vec = vec![];

        // We do the two checks to ignore diagonal lines (part1).
        if is_vertical {
            for y in self.start.y..=self.end.y {
                vec.push(Point { x: self.start.x, y })
            }
        } else if is_horizontal {
            for x in self.start.x..=self.end.x {
                vec.push(Point { x, y: self.start.y })
            }
        } else if include_diagonals {
            // Diagonal (45 degree) case
            let dx = self.end.x - self.start.x;
            let dy = self.end.y - self.start.y;
            let slope = dy / dx;

            for x in self.start.x..=self.end.x {
                vec.push(Point {
                    x,
                    y: self.start.y + slope * (x - self.start.x),
                })
            }
        }

        vec.into_iter()
    }
}
