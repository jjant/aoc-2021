#[derive(Clone)]
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
    fn step(&mut self) -> u32 {
        let width = self.width;
        let height = self.height;

        for x in 0..width {
            for y in 0..height {
                self.inc(x as isize, y as isize)
            }
        }

        let mut res = 0;
        for x in 0..width {
            for y in 0..height {
                res += self.reset_flashed(x as isize, y as isize);
            }
        }

        res
    }

    fn flash(&mut self, x: isize, y: isize) {
        if let Some(e) = self.get_mut(x, y) {
            e.flashed = true;

            for (x, y) in self.neighbors(x, y) {
                self.inc(x, y)
            }
        }
    }

    fn reset_flashed(&mut self, x: isize, y: isize) -> u32 {
        if let Some(e) = self.get_mut(x, y) {
            let res = if e.flashed { 1 } else { 0 };
            e.level = if e.flashed { 0 } else { e.level };
            e.flashed = false;

            res
        } else {
            0
        }
    }

    fn inc(&mut self, x: isize, y: isize) {
        if let Some(e) = self.get_mut(x, y) {
            e.level += 1;

            if e.level > 9 && !e.flashed {
                self.flash(x, y);
            }
        }
    }

    fn neighbors(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ]
        .iter()
        .map(|(dx, dy)| ((x + dx), (y + dy)))
        .collect()
    }

    fn parse(lines: &[&str]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| Energy {
                        level: c.to_digit(10).unwrap(),
                        flashed: false,
                    })
                    .collect()
            })
            .collect();

        Self {
            width,
            height,
            grid,
        }
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Energy> {
        let in_bounds =
            x >= 0 && x as usize <= self.width - 1 && y >= 0 && y as usize <= self.height - 1;

        if in_bounds {
            Some(&mut self.grid[y as usize][x as usize])
        } else {
            None
        }
    }
}

fn part1(lines: &[&str]) -> u32 {
    let mut grid = Grid::parse(lines);
    let steps = 100;

    let mut res = 0;
    for _ in 0..steps {
        res += grid.step();
    }

    res
}

fn part2(lines: &[&str]) -> u32 {
    let mut grid = Grid::parse(lines);

    let mut step = 0;
    loop {
        let all_zero = grid
            .grid
            .iter()
            .flat_map(|row| row.iter().map(|e| e.level))
            .all(|lvl| lvl == 0);

        if all_zero {
            break;
        }

        grid.step();
        step += 1;
    }

    step
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let lines = input_file.lines().collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(&lines));
    println!("Part 2:");
    println!("\t{}", part2(&lines));
}
