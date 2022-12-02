use super::read_lines;

pub fn solve() {
    let map: Vec<_> = read_lines("inputs/d15/0.txt")
        .expect("Could not find input for day 15!")
        .filter_map(|line| {
            line.ok().map(|line| {
                line.chars()
                    .map(|chr| chr.to_digit(10).expect("Invalid input!"))
                    .collect::<Vec<_>>()
            })
        })
        .collect();

    println!("Day 15 - First: (Partially correct. Too lazy for a proper priority queue)");
    solve_first(&map);
    println!("Day 15 - Secnd: (Partially correct. Too lazy for a proper priority queue)");
    solve_secnd(&map);
}

fn solve_first(map: &[Vec<u32>]) {
    let mut costs = map.to_owned();
    costs.iter_mut().flatten().for_each(|v| *v = u32::MAX);

    let ymax = map.len();
    let xmax = map[0].len();
    assert_eq!(ymax, xmax);

    costs[0][0] = 0;
    for x in 1..xmax {
        costs[0][x] = costs[0][x - 1] + map[0][x];
    }
    for y in 1..ymax {
        costs[y][0] = costs[y - 1][0] + map[y][0];
    }

    for i in 1..xmax {
        for x in 1..=i {
            costs[i][x] = costs[i][x - 1].min(costs[i - 1][x]) + map[i][x];
        }

        for y in 1..=i {
            costs[y][i] = costs[y - 1][i].min(costs[y][i - 1]) + map[y][i];
        }
    }

    println!("The answer is {}", costs[ymax - 1][xmax - 1]);
}

fn solve_secnd(map: &[Vec<u32>]) {
    let mut new_map = vec![Vec::new(); map.len() * 5];
    for i in 0..5 {
        for (y, row) in map.iter().enumerate() {
            new_map[i * map.len() + y].resize(row.len() * 5, 0);
            for j in 0..5 {
                for (x, value) in row.iter().enumerate() {
                    new_map[i * map.len() + y][j * row.len() + x] =
                        (*value - 1 + j as u32 + i as u32) % 9 + 1;
                }
            }
        }
    }

    solve_first(&new_map);
}
