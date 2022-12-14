use std::collections::HashSet;

fn parse_rock(line: &str, grid: &mut HashSet<(u16, u16)>) {
    let mut prev: Option<(u16, u16)> = None;

    for coords in line.split(" -> ") {
        let (x, y) = coords.split_once(',').expect("invalid input");

        let x = x.parse::<u16>().expect("invalid input");
        let y = y.parse::<u16>().expect("invalid input");

        if let Some(prev) = prev {
            assert!(prev.0 == x || prev.1 == y);

            let x0 = prev.0.min(x);
            let x1 = prev.0.max(x);

            let y0 = prev.1.min(y);
            let y1 = prev.1.max(y);

            for i in x0..=x1 {
                for j in y0..=y1 {
                    grid.insert((i, j));
                }
            }
        }

        prev = Some((x, y))
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut grid = HashSet::new();

    for line in input {
        parse_rock(line, &mut grid);
    }

    let mut sand_count = 0;

    loop {
        let mut start = (500, 0);

        while start.1 < 500 {
            if !grid.contains(&(start.0, start.1 + 1)) {
                start = (start.0, start.1 + 1)
            } else if !grid.contains(&(start.0 - 1, start.1 + 1)) {
                start = (start.0 - 1, start.1 + 1);
            } else if !grid.contains(&(start.0 + 1, start.1 + 1)) {
                start = (start.0 + 1, start.1 + 1)
            } else {
                grid.insert(start);
                break;
            }
        }

        if start.1 == 500 {
            break;
        }

        sand_count += 1;
    }

    sand_count
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    if !cfg!(debug_assertions) {
        let mut grid = HashSet::new();

        for line in input {
            parse_rock(line, &mut grid);
        }

        let mut sand_count = 0;
        let bottom = grid.iter().map(|pos| pos.1).max().unwrap_or(0) + 2;

        loop {
            let mut start = (500, 0);

            loop {
                if start.1 + 1 == bottom {
                    grid.insert(start);
                    break;
                } else if !grid.contains(&(start.0, start.1 + 1)) {
                    start = (start.0, start.1 + 1)
                } else if !grid.contains(&(start.0 - 1, start.1 + 1)) {
                    start = (start.0 - 1, start.1 + 1);
                } else if !grid.contains(&(start.0 + 1, start.1 + 1)) {
                    start = (start.0 + 1, start.1 + 1)
                } else {
                    grid.insert(start);
                    break;
                }
            }

            sand_count += 1;

            if start == (500, 0) {
                break;
            }
        }

        sand_count.to_string()
    } else {
        "disabled for debug build :(".to_string()
    }
}

impl_dayx!("14", solve1, solve2);
