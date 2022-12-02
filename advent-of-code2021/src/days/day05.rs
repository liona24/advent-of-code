use super::read_lines;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl TryFrom<&str> for Point {
    type Error = ();

    fn try_from(string: &str) -> Result<Point, Self::Error> {
        let mut splits = string.split(',').filter_map(|x| x.parse().ok());
        let x = splits.next().ok_or(())?;
        let y = splits.next().ok_or(())?;

        Ok(Point { x, y })
    }
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(string: &str) -> Result<Line, Self::Error> {
        let mut splits = string.split(' ');

        let first = splits.next().ok_or(())?;
        let secnd = splits.nth(1).ok_or(())?;

        Ok(Line {
            start: first.try_into()?,
            end: secnd.try_into()?,
        })
    }
}

pub fn solve() {
    let lines: Vec<Line> = read_lines("inputs/d05/0.txt")
        .expect("Could not find input for day 5!")
        .filter_map(|line| {
            line.ok()
                .map(|line| line.as_str().try_into().expect("Invalid input!"))
        })
        .collect();

    let width = lines
        .iter()
        .map(|line| line.start.x.max(line.end.x))
        .max()
        .expect("Need at least one value!") as usize
        + 1;
    let height = lines
        .iter()
        .map(|line| line.start.y.max(line.end.y))
        .max()
        .expect("Need at least one value!") as usize
        + 1;

    let mut field = vec![0; width * height];

    println!("Day 05 - First:");
    solve_first(&mut field, width, &lines);
    println!("Day 05 - Secnd:");
    solve_secnd(&mut field, width, &lines);
}

fn solve_first(field: &mut [u32], width: usize, lines: &[Line]) {
    for line in lines.iter() {
        let x0 = line.start.x as i32;
        let x1 = line.end.x as i32;

        let y0 = line.start.y as i32;
        let y1 = line.end.y as i32;

        if x0 == x1 {
            let y0 = line.start.y.min(line.end.y);
            let y1 = line.start.y.max(line.end.y);

            for i in y0..=y1 {
                field[i as usize * width + x0 as usize] += 1;
            }
        }

        if y0 == y1 {
            let x0 = line.start.x.min(line.end.x);
            let x1 = line.start.x.max(line.end.x);
            for i in x0..=x1 {
                field[y0 as usize * width + i as usize] += 1;
            }
        }
    }

    let count = field.iter().filter(|v| **v >= 2).count();
    println!("The answer is {}", count);
}

fn solve_secnd(field: &mut [u32], width: usize, lines: &[Line]) {
    for line in lines.iter() {
        let x0 = line.start.x as i32;
        let x1 = line.end.x as i32;

        let y0 = line.start.y as i32;
        let y1 = line.end.y as i32;

        if (y1 - y0).abs() == (x1 - x0).abs() {
            let sx = (x1 - x0).signum();
            let sy = (y1 - y0).signum();

            for i in 0..=(x1 - x0).abs() {
                field[(y0 + i * sy) as usize * width + (x0 + i * sx) as usize] += 1;
            }
        }
    }

    let count = field.iter().filter(|v| **v >= 2).count();
    println!("The answer is {}", count);
}
