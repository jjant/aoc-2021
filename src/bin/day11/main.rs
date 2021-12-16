#[derive(Clone, Copy)]
struct Energy {
    level: u32,
    flashed: bool,
}

struct Grid {
    grid: Vec<Vec<Energy>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let energy = Energy {
            level: 0,
            flashed: false,
        };
        let grid = vec![vec![energy; width]; height];

        Self {
            grid,
            width,
            height,
        }
    }

    fn step(&mut self) -> u32 {
        let width = self.width;
        let height = self.height;

        for x in 0..width {
            for y in 0..height {
                self.inc(x, y)
            }
        }

        let mut res = 0;
        for x in 0..width {
            for y in 0..height {
                res += self.reset_flashed(x, y);
            }
        }

        res
    }

    fn flash(&mut self, x: usize, y: usize) {
        self.grid[y][x].flashed = true;

        for (x, y) in self.neighbors(x, y) {
            self.inc(x, y)
        }
    }

    fn reset_flashed(&mut self, x: usize, y: usize) -> u32 {
        let e = &mut self.grid[y][x];
        let res = if e.flashed { 1 } else { 0 };
        e.level = if e.flashed { 0 } else { e.level };
        e.flashed = false;

        res
    }

    fn inc(&mut self, x: usize, y: usize) {
        let e = &mut self.grid[y][x];
        e.level += 1;

        if e.level > 9 && !e.flashed {
            self.flash(x, y);
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let first_col = x == 0;
        let last_col = x == self.width - 1;
        let first_row = y == 0;
        let last_row = y == self.height - 1;

        let left = (!first_col, (-1, 0));
        let right = (!last_col, (1, 0));
        let top = (!first_row, (0, -1));
        let bottom = (!last_row, (0, 1));
        let top_left = (!first_row && !first_col, (-1, -1));
        let top_right = (!first_row && !last_col, (1, -1));
        let bottom_left = (!last_row && !first_col, (-1, 1));
        let bottom_right = (!last_row && !last_col, (1, 1));

        [
            left,
            right,
            top,
            bottom,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ]
        .iter()
        .filter_map(|(b, (dx, dy))| {
            if *b {
                Some(((x as isize + dx) as usize, (y as isize + dy) as usize))
            } else {
                None
            }
        })
        .collect()
    }
}

fn part1(lines: &[&str]) -> u32 {
    let steps = 100;
    let mut grid = Grid::new(lines.len(), lines[0].len());

    for (y, line) in lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .enumerate()
    {
        for (x, cell) in line.enumerate() {
            grid.grid[y][x] = Energy {
                level: cell,
                flashed: false,
            }
        }
    }

    let mut res = 0;
    for _ in 0..steps {
        res += grid.step();
    }

    res
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let lines = input_file.lines().collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&lines));
}
