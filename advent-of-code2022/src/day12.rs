use std::collections::BinaryHeap;

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<usize>,
    size: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
}

impl From<&[String]> for Grid {
    fn from(value: &[String]) -> Self {
        let width = value.first().map(|line| line.len()).unwrap_or(0) + 2;
        let mut height = 1usize;
        let mut values: Vec<usize> = vec![usize::MAX; width];

        for line in value {
            values.push(usize::MAX);
            values.extend(line.as_bytes().iter().map(|x| *x as usize));
            values.push(usize::MAX);
            height += 1;
        }

        height += 1;
        for _ in 0..width {
            values.push(usize::MAX);
        }

        let mut start = (0, 0);
        let mut end = (0, 0);

        for i in 0..height {
            for j in 0..width {
                if values[i * width + j] == 'S' as usize {
                    start = (j, i);
                    values[i * width + j] = 'a' as usize;
                }
                if values[i * width + j] == 'E' as usize {
                    end = (j, i);
                    values[i * width + j] = 'z' as usize;
                }
            }
        }

        Self {
            values,
            size: (width, height),
            start,
            end,
        }
    }
}

impl Grid {
    fn at(&self, pos: (usize, usize)) -> &usize {
        &self.values[pos.1 * self.size.0 + pos.0]
    }
}

#[derive(Debug, Clone)]
struct Element<'g> {
    pos: (usize, usize),
    grid: &'g Grid,
}

impl<'g> Element<'g> {
    fn dist(&self) -> usize {
        self.pos.0.abs_diff(self.grid.end.0) + self.pos.1.abs_diff(self.grid.end.1)
    }
}

impl<'g> PartialEq for Element<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<'g> Eq for Element<'g> {}

impl<'g> PartialOrd for Element<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'g> Ord for Element<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let dist_self = self.dist();
        let dist_other = other.dist();

        dist_other.cmp(&dist_self)
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let grid: Grid = input.into();

    let mut q = BinaryHeap::new();
    q.push(Element {
        pos: grid.start,
        grid: &grid,
    });
    let mut path_lengths = vec![usize::MAX; grid.values.len()];
    path_lengths[grid.start.1 * grid.size.0 + grid.start.0] = 0;

    while let Some(top) = q.pop() {
        let z = *grid.at(top.pos);

        let len = path_lengths[top.pos.1 * grid.size.0 + top.pos.0];

        if top.pos == grid.end {
            break;
        }

        for dx in -1..=1i64 {
            for dy in -1..=1i64 {
                if dx.abs() + dy.abs() != 1 {
                    continue;
                }

                let x = (top.pos.0 as i64 + dx) as usize;
                let y = (top.pos.1 as i64 + dy) as usize;

                if z + 1 >= *grid.at((x, y)) {
                    if len + 1 < path_lengths[y * grid.size.0 + x] {
                        path_lengths[y * grid.size.0 + x] = len + 1;

                        let new = Element {
                            pos: (x, y),
                            grid: &grid,
                        };

                        q.push(new);
                    }
                }
            }
        }
    }

    path_lengths[grid.end.1 * grid.size.0 + grid.end.0]
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let grid: Grid = input.into();

    let mut q = BinaryHeap::new();
    q.push(Element {
        pos: grid.end,
        grid: &grid,
    });
    let mut path_lengths = vec![usize::MAX; grid.values.len()];
    path_lengths[grid.end.1 * grid.size.0 + grid.end.0] = 0;

    while let Some(top) = q.pop() {
        let z = *grid.at(top.pos);

        let len = path_lengths[top.pos.1 * grid.size.0 + top.pos.0];

        for dx in -1..=1i64 {
            for dy in -1..=1i64 {
                if dx.abs() + dy.abs() != 1 {
                    continue;
                }

                let x = (top.pos.0 as i64 + dx) as usize;
                let y = (top.pos.1 as i64 + dy) as usize;

                if grid.at((x, y)).checked_add(1).map_or(false, |v| v >= z) {
                    if len + 1 < path_lengths[y * grid.size.0 + x] {
                        path_lengths[y * grid.size.0 + x] = len + 1;

                        let new = Element {
                            pos: (x, y),
                            grid: &grid,
                        };

                        q.push(new);
                    }
                }
            }
        }
    }

    path_lengths
        .into_iter()
        .zip(grid.values.into_iter())
        .filter(|(_len, value)| *value == 'a' as usize)
        .map(|(len, _value)| len)
        .min()
        .unwrap()
}

impl_dayx!("12", solve1, solve2);
