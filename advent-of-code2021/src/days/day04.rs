use super::read_lines;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Copy, Clone)]
struct Line {
    markers: u8,
}

#[derive(Debug, Clone)]
struct BoardCollection {
    rows: Vec<Line>,
    cols: Vec<Line>,
    numbers: Vec<[i32; 5 * 5]>,
    refs: HashMap<i32, Vec<(usize, usize, usize)>>,
    finished: BTreeSet<usize>,
}

impl Line {
    fn new() -> Self {
        Self { markers: 0x1f }
    }

    fn is_full(&self) -> bool {
        self.markers == 0
    }

    fn mark(&mut self, i: usize) {
        self.markers &= !(1u8 << i);
    }
}

impl BoardCollection {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            cols: Vec::new(),
            numbers: Vec::new(),
            refs: HashMap::new(),
            finished: BTreeSet::new(),
        }
    }

    fn mark_row(&mut self, draw: i32) -> Option<usize> {
        if let Some(refs) = self.refs.get(&(draw as i32)) {
            let mut winning_board = None;

            for (board_idx, row, col) in refs {
                if self.finished.contains(board_idx) {
                    continue;
                }

                self.numbers[*board_idx][row * 5 + col] = -1;
                let line = &mut self.rows[board_idx * 5 + row];
                line.mark(*col);
                if line.is_full() {
                    self.finished.insert(*board_idx);
                    winning_board = Some(*board_idx);
                }
            }

            winning_board
        } else {
            None
        }
    }

    fn mark_col(&mut self, draw: i32) -> Option<usize> {
        if let Some(refs) = self.refs.get(&(draw as i32)) {
            let mut winning_board = None;

            for (board_idx, row, col) in refs {
                if self.finished.contains(board_idx) {
                    continue;
                }

                self.numbers[*board_idx][row * 5 + col] = -1;
                let line = &mut self.cols[board_idx * 5 + col];
                line.mark(*row);
                if line.is_full() {
                    self.finished.insert(*board_idx);
                    winning_board = Some(*board_idx);
                }
            }

            winning_board
        } else {
            None
        }
    }
}

pub fn solve() {
    let mut lines = read_lines("inputs/d04/0.txt").expect("Could not find input for day 4!");

    let draws: Vec<u32> = lines
        .next()
        .expect("Draws are required")
        .expect("Could not read line!")
        .split(',')
        .map(|n| n.parse().expect("Invalid input!"))
        .collect();
    let mut boards = BoardCollection::new();

    let mut i = 0;

    for line in lines {
        let line = line.expect("Could not read line!");
        if line.is_empty() {
            boards.numbers.push([0; 5 * 5]);
            i = 0;
        } else {
            let current_board_idx = boards.numbers.len() - 1;
            let numbers = boards.numbers.last_mut().unwrap();

            // lazy version, will push n of each assuming the board is squared and the input is correct
            boards.rows.push(Line::new());
            boards.cols.push(Line::new());

            for (j, n) in line
                .split(' ')
                .filter_map(|n| n.parse::<u32>().ok())
                .enumerate()
            {
                numbers[i * 5 + j] = n as i32;
                boards
                    .refs
                    .entry(n as i32)
                    .and_modify(|v| v.push((current_board_idx, i, j)))
                    .or_insert_with(|| vec![(current_board_idx, i, j)]);
            }
            i += 1;
        }
    }

    println!("Day 04 - First:");
    solve_first(&draws, &mut boards);
    println!("Day 04 - Secnd:");
    solve_secnd(&draws, &mut boards);
}

fn solve_first(draws: &[u32], boards: &mut BoardCollection) {
    let mut winning_board = None;

    for draw in draws {
        let win1 = boards.mark_col(*draw as i32);
        let win2 = boards.mark_row(*draw as i32);

        if let Some(board_idx) = win1 {
            winning_board = Some((board_idx, *draw));
            break;
        }
        if let Some(board_idx) = win2 {
            winning_board = Some((board_idx, *draw));
            break;
        }
    }

    let winning_board = winning_board.expect("Should have at least one winning board!");
    let sum: i32 = boards.numbers[winning_board.0]
        .iter()
        .filter(|&v| *v != -1)
        .sum();

    println!("The answer is {}", (sum as u32) * winning_board.1);
}

fn solve_secnd(draws: &[u32], boards: &mut BoardCollection) {
    let mut winning_board = None;

    for draw in draws {
        let win1 = boards.mark_col(*draw as i32);
        let win2 = boards.mark_row(*draw as i32);

        let mut tmp = None;
        if let Some(board_idx) = win1 {
            tmp = Some((board_idx, *draw));
        }
        if let Some(board_idx) = win2 {
            tmp = Some((board_idx, *draw));
        }

        if boards.finished.len() == boards.numbers.len() {
            winning_board = tmp;
            break;
        }
    }

    let winning_board = winning_board.expect("Should have at least one winning board!");
    let sum: i32 = boards.numbers[winning_board.0]
        .iter()
        .filter(|&v| *v != -1)
        .sum();

    println!("The answer is {}", (sum as u32) * winning_board.1);
}
