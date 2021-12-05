use std::vec;

struct World(Vec<Vec<u32>>);

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

fn part1(lines: &[Line]) -> u32 {
    let width = lines
        .iter()
        .map(|line| line.start.x.max(line.end.x))
        .max()
        .unwrap() as usize
        + 1;
    let height = lines
        .iter()
        .map(|line| line.start.y.max(line.end.y))
        .max()
        .unwrap() as usize
        + 1;

    let mut world = World::new(width, height);

    for line in lines {
        for point in line.iter() {
            world.mark(&point)
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

// fn part2(numbers_to_draw: &[u32], boards: &mut Vec<Board>) -> u32 {}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let parsed_input = input_file
        .split('\n')
        .map(|line| Line::parse(line))
        .collect::<Option<Vec<_>>>();

    println!("Part 1:");
    println!("\t{}", part1(&parsed_input.unwrap()));
    println!("Part 2:");
    // println!("\t{}", part2(&parsed_input.0, &mut parsed_input.1));
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    // Parses a string like:
    //  "0,9"
    pub fn parse(input: &str) -> Option<Self> {
        let (str_x, str_y) = input.split_once(',')?;

        let x = str_x.parse().ok()?;
        let y = str_y.parse().ok()?;

        Some(Self { x, y })
    }
}
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    // Parses a string like:
    //  "0,9 -> 5,9"
    pub fn parse(input: &str) -> Option<Self> {
        let (str_p1, str_p2) = input.split_once(" -> ")?;

        let start = Point::parse(str_p1)?;
        let end = Point::parse(str_p2)?;

        Some(Self { start, end })
    }

    pub fn iter(&self) -> impl Iterator<Item = Point> {
        let is_vertical = self.start.x == self.end.x;
        let is_horizontal = self.start.y == self.end.y;
        let mut vec = vec![];

        // We do the two checks to ignore diagonal lines.
        if is_vertical {
            let (min_y, max_y) = sort2(self.start.y, self.end.y);

            for y in min_y..=max_y {
                vec.push(Point { x: self.start.x, y })
            }
        } else if is_horizontal {
            let (min_x, max_x) = sort2(self.start.x, self.end.x);

            for x in min_x..=max_x {
                vec.push(Point { x, y: self.start.y })
            }
        }

        vec.into_iter()
    }
}

fn sort2(a: u32, b: u32) -> (u32, u32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
