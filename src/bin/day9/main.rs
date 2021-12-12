#[derive(Debug)]
struct Grid<T> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    fn extend<F>(&self, f: F) -> Grid<T>
    where
        F: Fn(&T, &[&T]) -> T,
    {
        let mut new_grid = Grid::new(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.get(x, y);
                let neighbors = self.neighbors(x, y);

                let res = f(val, &neighbors);

                new_grid.grid.push(res);
            }
        }

        new_grid
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<&T> {
        let mut neighbors = vec![];

        let left = if x > 0 { Some((-1, 0)) } else { None };
        let right = if x < self.width - 1 {
            Some((1, 0))
        } else {
            None
        };
        let top = if y > 0 { Some((0, -1)) } else { None };
        let bottom = if y < self.height - 1 {
            Some((0, 1))
        } else {
            None
        };

        for dir in &[left, right, top, bottom] {
            if let Some((dx, dy)) = dir {
                neighbors.push(self.get((x as i32 + *dx) as usize, ((y as i32) + *dy) as usize));
            }
        }

        neighbors
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[x + y * self.width]
    }
}

impl<T: Copy> Grid<T> {
    pub fn from_data(width: usize, height: usize, data: &[T]) -> Self {
        let mut grid = Grid::new(width, height);

        if data.len() != width * height {
            panic!(
                "Incorrect len. Expected: {}. Found: {}",
                width * height,
                data.len()
            );
        }

        for d in data {
            grid.grid.push(*d);
        }

        grid
    }
}

fn part1(width: usize, height: usize, data: &[u32]) -> u32 {
    let grid = Grid::from_data(width, height, data);

    let new_grid = grid.extend(|val, neighbors| {
        let val_is_minimum = neighbors.iter().all(|neighbor| *neighbor > val);

        if val_is_minimum {
            val + 1
        } else {
            0
        }
    });

    let res: u32 = new_grid.grid.iter().sum::<u32>();

    res
}

fn main() {
    let input_file: &str = include_str!("input.txt");
    let width = input_file.lines().next().unwrap().len();
    let height = input_file.lines().count();

    let data = input_file
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1:");
    println!("\t{}", part1(width, height, &data));
}
