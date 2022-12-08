fn score_at(grid: &[i32], width: usize, height: usize, x: usize, y: usize) -> u64 {
    let width_ext = width + 2;
    let height_ext = height + 2;

    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;

    let x = x + 1;
    let y = y + 1;

    let prev = grid[y * width_ext + x];
    for j in x + 1..width_ext - 1 {
        let current = grid[y * width_ext + j];
        if prev > current {
            count0 += 1;
        }

        if prev <= current {
            count0 += 1;
            break;
        }
    }

    for j in (1..x).rev() {
        let current = grid[y * width_ext + j];
        if prev > current {
            count1 += 1;
        }

        if prev <= current {
            count1 += 1;
            break;
        }
    }

    for i in y + 1..height_ext - 1 {
        let current = grid[i * width_ext + x];
        if prev > current {
            count2 += 1;
        }

        if prev <= current {
            count2 += 1;
            break;
        }
    }

    for i in (1..y).rev() {
        let current = grid[i * width_ext + x];
        if prev > current {
            count3 += 1;
        }

        if prev <= current {
            count3 += 1;
            break;
        }
    }

    count0 * count1 * count2 * count3
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let width = input.first().map(|line| line.len()).unwrap_or(0);
    let height = input.len();

    let width_ext = width + 2;
    let height_ext = height + 2;

    let mut grid = vec![-1; width_ext * height_ext];
    let mut markers = vec![false; width_ext * height_ext];

    for (i, line) in input.iter().enumerate() {
        let i = i + 1;
        for (j, tree_height) in line.trim_end().chars().enumerate() {
            let j = j + 1;
            grid[i * width_ext + j] = tree_height as i32;
        }
    }

    for i in 0..height {
        let mut prev = -1;
        for j in 0..width {
            let current = grid[(i + 1) * width_ext + j + 1];
            if prev < current {
                markers[(i + 1) * width_ext + j + 1] = true;
            }

            prev = prev.max(current);
        }

        prev = -1;
        for j in (1..=width).rev() {
            let current = grid[(i + 1) * width_ext + j];
            if prev < current {
                markers[(i + 1) * width_ext + j] = true;
            }

            prev = prev.max(current);
        }
    }

    for j in 0..width {
        let mut prev = -1;
        for i in 0..height {
            let current = grid[(i + 1) * width_ext + j + 1];
            if prev < current {
                markers[(i + 1) * width_ext + j + 1] = true;
            }

            prev = prev.max(current);
        }

        prev = -1;
        for i in (1..=height).rev() {
            let current = grid[i * width_ext + j + 1];
            if prev < current {
                markers[i * width_ext + j + 1] = true;
            }

            prev = prev.max(current);
        }
    }

    markers.into_iter().filter(|marker| *marker).count()
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let width = input.first().map(|line| line.len()).unwrap_or(0);
    let height = input.len();

    let width_ext = width + 2;
    let height_ext = height + 2;

    let mut grid = vec![-1; width_ext * height_ext];

    for (i, line) in input.iter().enumerate() {
        let i = i + 1;
        for (j, tree_height) in line.trim_end().chars().enumerate() {
            let j = j + 1;
            grid[i * width_ext + j] = tree_height as i32;
        }
    }

    let mut score = 0;
    for i in 0..height {
        for j in 0..width {
            score = score.max(score_at(&grid, width, height, j, i));
        }
    }

    score
}

impl_dayx!("08", solve1, solve2);
