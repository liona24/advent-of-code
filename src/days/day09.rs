use super::read_lines;
use std::collections::HashSet;

pub fn solve() {
    let height_map: Vec<Vec<u8>> = read_lines("inputs/d09/0.txt")
        .expect("Could not find input for day 9!")
        .filter_map(|line| {
            line.ok()
                .map(|line| line.bytes().map(|chr| chr - b'0').collect())
        })
        .collect();

    println!("Day 09 - First:");
    let centers = solve_first(&height_map);
    println!("Day 09 - Secnd:");
    solve_secnd(&height_map, centers);
}

fn solve_first(height_map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut risk_level = 0;

    let mut center_positions = Vec::new();

    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            let center = height_map[y][x];
            let mut is_min = true;

            let y_min = 0.max(y as i64 - 1) as usize;
            let x_min = 0.max(x as i64 - 1) as usize;
            let y_max = (height_map.len() - 1).min(y + 1);
            let x_max = (height_map[y].len() - 1).min(x + 1);

            #[allow(clippy::needless_range_loop)]
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    if height_map[y][x] < center {
                        is_min = false;
                        break;
                    }
                }

                if !is_min {
                    break;
                }
            }

            if is_min {
                center_positions.push((x, y));
                risk_level += (center as usize) + 1;
            }
        }
    }

    println!("The answer is {}", risk_level);

    center_positions
}

fn solve_secnd(height_map: &[Vec<u8>], centers: Vec<(usize, usize)>) {
    let mut l1 = 0;
    let mut l2 = 0;
    let mut l3 = 0;

    for (x, y) in centers.into_iter() {
        let mut size = 0;

        let mut markers = HashSet::new();
        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            let y_min = 0.max(y as i64 - 1) as usize;
            let x_min = 0.max(x as i64 - 1) as usize;
            let y_max = (height_map.len() - 1).min(y + 1);
            let x_max = (height_map[y].len() - 1).min(x + 1);

            #[allow(clippy::needless_range_loop)]
            for y in y_min..=y_max {
                if height_map[y][x] != 9 && !markers.contains(&(x, y)) {
                    markers.insert((x, y));
                    stack.push((x, y));
                    size += 1;
                }
            }
            for x in x_min..=x_max {
                if height_map[y][x] != 9 && !markers.contains(&(x, y)) {
                    markers.insert((x, y));
                    stack.push((x, y));
                    size += 1;
                }
            }
        }

        if size > l1 {
            l3 = l2;
            l2 = l1;
            l1 = size;
        } else if size > l2 {
            l3 = l2;
            l2 = size;
        } else if size > l3 {
            l3 = size;
        }
    }

    println!("The answer is {}", l1 * l2 * l3);
}
