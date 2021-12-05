fn part1(numbers_to_draw: &[u32], boards: &mut [Board]) -> u32 {
    for number in numbers_to_draw {
        for board in boards.iter_mut() {
            if let Some(result) = board.mark(*number) {
                return result;
            }
        }
    }

    panic!("Wrong input, no winners determined");
}

fn part2(numbers_to_draw: &[u32], boards: &mut Vec<Board>) -> u32 {
    for number in numbers_to_draw {
        let mut i = 0;
        while i < boards.len() {
            // This should be easier with drain_filter?
            if let Some(result) = boards[i].mark(*number) {
                if boards.len() == 1 {
                    return result;
                }
                boards.remove(i);
            } else {
                i += 1;
            }
        }
    }

    panic!("Wrong input, last board doesn't actually win")
}

fn main() {
    let input_file: &str = include_str!("input.txt");

    let mut parsed_input = parse(input_file);

    println!("Part 1:");
    println!("\t{}", part1(&parsed_input.0, &mut parsed_input.1));
    println!("Part 2:");
    println!("\t{}", part2(&parsed_input.0, &mut parsed_input.1));
}

const SIZE: usize = 5;

type BoardArray = [[(u32, bool); SIZE]; SIZE];

struct Board(BoardArray);

impl Board {
    pub fn mark(&mut self, number: u32) -> Option<u32> {
        let mut marked_coords = None;

        // The break in this loop assumes that boards have no repeated numbers
        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, (cell, marked)) in row.iter_mut().enumerate() {
                if *cell == number {
                    *marked = true;

                    marked_coords = Some((x, y));
                    break;
                }
            }
        }

        let (x, y) = marked_coords?;

        Some(self.check_win(x, y)? * self.0[y][x].0)
    }

    pub fn check_win(&self, x: usize, y: usize) -> Option<u32> {
        let row_won = self.0[y].iter().all(|(_, marked)| *marked);

        if row_won {
            return Some(self.sum_unmarked());
        }
        let mut col_won = true;
        for y in 0..SIZE {
            let marked = self.0[y][x].1;
            if !marked {
                col_won = false;
                break;
            }
        }
        if col_won {
            return Some(self.sum_unmarked());
        } else {
            return None;
        }
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.0
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|(n, b)| if !*b { Some(*n) } else { None })
            })
            .sum()
    }

    pub fn parse(input: &str) -> Option<Self> {
        let rows = input.split('\n').map(|s| s.split_whitespace());
        let mut board = [[(0, false); SIZE]; SIZE];

        for (y, row) in rows.enumerate() {
            for (x, n) in row.enumerate() {
                board[y][x].0 = n.parse().unwrap();
            }
        }

        Some(Board(board))
    }
}

fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut it = input.split("\n\n");

    let numbers_to_draw = it
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let boards = it
        .map(|board_str| Board::parse(board_str).unwrap())
        .collect();

    (numbers_to_draw, boards)
}
