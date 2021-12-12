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

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, &T)> {
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
                let n_x = (x as i32 + *dx) as usize;
                let n_y = (y as i32 + *dy) as usize;

                neighbors.push((n_x, n_y, self.get(n_x, n_y)));
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

impl Grid<u32> {
    fn find_minimums(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.neighbors(x, y);

                if neighbors
                    .iter()
                    .all(|(_, _, neighbor)| neighbor > &self.get(x, y))
                {
                    res.push((x, y))
                }
            }
        }

        res
    }

    fn basin(&self, x: usize, y: usize) -> Vec<&u32> {
        let mut res = vec![];
        self.dfs(x, y, &mut vec![false; self.width * self.height], &mut res);

        res
    }

    fn dfs<'a>(&'a self, x: usize, y: usize, visited: &mut [bool], res: &mut Vec<&'a u32>) {
        let current = self.get(x, y);
        let neighbors = self.neighbors(x, y);

        visited[x + y * self.width] = true;
        res.push(current);

        let real_neighbors = neighbors
            .iter()
            .filter(|(_, _, neighbor)| **neighbor < 9 && *neighbor > current);

        for (n_x, n_y, _) in real_neighbors {
            if !visited[n_x + n_y * self.width] {
                self.dfs(*n_x, *n_y, visited, res);
            }
        }
    }
}

fn part1(width: usize, height: usize, data: &[u32]) -> u32 {
    let grid = Grid::from_data(width, height, data);

    grid.find_minimums()
        .iter()
        .map(|(x, y)| grid.get(*x, *y) + 1)
        .sum()
}

fn part2(width: usize, height: usize, data: &[u32]) -> u32 {
    let grid = Grid::from_data(width, height, data);

    let minimums: Vec<(usize, usize)> = grid.find_minimums();

    let mut basin_lengths = minimums
        .iter()
        .map(|(x, y)| grid.basin(*x, *y).len())
        .collect::<Vec<_>>();

    basin_lengths.sort_unstable();

    let res: usize = basin_lengths.iter().rev().take(3).product();

    res as u32
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
    println!("Part 2:");
    println!("\t{}", part2(width, height, &data));
}
