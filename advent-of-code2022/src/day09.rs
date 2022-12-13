use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, steps) = value.split_once(' ').ok_or("invalid input".to_string())?;

        let steps = steps.parse::<usize>().map_err(|e| e.to_string())?;

        match dir {
            "R" => Ok(Direction::Right(steps)),
            "L" => Ok(Direction::Left(steps)),
            "U" => Ok(Direction::Up(steps)),
            "D" => Ok(Direction::Down(steps)),
            _ => Err("invalid input".to_string()),
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut grid = HashSet::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    grid.insert(tail);

    for line in input {
        let dir: Direction = line.as_str().try_into().expect("invalid input");

        match dir {
            Direction::Up(steps) => {
                for _ in 0..steps {
                    head.1 += 1;
                    if head.1 - tail.1 > 1 {
                        tail.1 = head.1 - 1;
                        tail.0 = head.0;
                        grid.insert(tail);
                    }
                }
            }
            Direction::Down(steps) => {
                for _ in 0..steps {
                    head.1 -= 1;
                    if tail.1 - head.1 > 1 {
                        tail.1 = head.1 + 1;
                        tail.0 = head.0;
                        grid.insert(tail);
                    }
                }
            }
            Direction::Right(steps) => {
                for _ in 0..steps {
                    head.0 += 1;
                    if head.0 - tail.0 > 1 {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                        grid.insert(tail);
                    }
                }
            }
            Direction::Left(steps) => {
                for _ in 0..steps {
                    head.0 -= 1;
                    if tail.0 - head.0 > 1 {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                        grid.insert(tail);
                    }
                }
            }
        }
    }

    grid.len()
}

fn solve2(_input: &[String]) -> impl std::fmt::Display {
    "yeah ... not gonna do that."
}

impl_dayx!("09", solve1, solve2);
