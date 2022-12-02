use super::read_lines;
use std::collections::BTreeSet;

pub fn solve() {
    let inputs: Vec<Vec<_>> = read_lines("inputs/d11/0.txt")
        .expect("Could not find input for day 11!")
        .filter_map(|line| {
            line.ok().map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid input!"))
                    .collect()
            })
        })
        .collect();

    println!("Day 11 - First:");
    solve_first(&inputs);
    println!("Day 11 - Secnd:");
    solve_secnd(&inputs);
}

fn solve_first(inputs: &[Vec<u32>]) {
    let mut inputs = inputs.to_owned();
    let mut count = 0;

    for _ in 0..100 {
        let mut flashed = BTreeSet::new();

        for i in inputs.iter_mut().flatten() {
            *i += 1;
        }

        loop {
            let mut to_flash = BTreeSet::new();

            for i in 0..inputs.len() {
                for j in 0..inputs[i].len() {
                    if inputs[i][j] >= 10 && flashed.insert((i, j)) {
                        to_flash.insert((i, j));

                        let i = i as i64;
                        let j = j as i64;

                        let i_mx = inputs.len() as i64 - 1;
                        let j_mx = inputs.len() as i64 - 1;

                        for i in 0.max(i - 1)..=i_mx.min(i + 1) {
                            for j in 0.max(j - 1)..=j_mx.min(j + 1) {
                                inputs[i as usize][j as usize] += 1;
                            }
                        }
                    }
                }
            }

            if to_flash.is_empty() {
                break;
            }
        }

        count += flashed.len();

        for (i, j) in flashed.into_iter() {
            inputs[i][j] = 0;
        }
    }

    println!("The answer is {}", count);
}

fn solve_secnd(inputs: &[Vec<u32>]) {
    let mut inputs = inputs.to_owned();
    let mut count = 0;

    loop {
        count += 1;
        let mut flashed = BTreeSet::new();

        for i in inputs.iter_mut().flatten() {
            *i += 1;
        }

        loop {
            let mut to_flash = BTreeSet::new();

            for i in 0..inputs.len() {
                for j in 0..inputs[i].len() {
                    if inputs[i][j] >= 10 && flashed.insert((i, j)) {
                        to_flash.insert((i, j));

                        let i = i as i64;
                        let j = j as i64;

                        let i_mx = inputs.len() as i64 - 1;
                        let j_mx = inputs.len() as i64 - 1;

                        for i in 0.max(i - 1)..=i_mx.min(i + 1) {
                            for j in 0.max(j - 1)..=j_mx.min(j + 1) {
                                inputs[i as usize][j as usize] += 1;
                            }
                        }
                    }
                }
            }

            if to_flash.is_empty() {
                break;
            }
        }

        if flashed.len() == 100 {
            break;
        }

        for (i, j) in flashed.into_iter() {
            inputs[i][j] = 0;
        }
    }

    println!("The answer is {}", count);
}
