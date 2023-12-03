use std::{collections::HashSet, rc::Rc};

#[derive(Debug)]
enum Glyph {
    Symbol(char),
    Number(Rc<u32>),
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Glyph>,
}

impl Grid {
    fn parse(input: &[String]) -> Option<Self> {
        use regex::Regex;

        let mut width = 0;
        let mut height = 0;
        let mut grid = Vec::new();

        let r = Regex::new(r"[0-9]+").unwrap();

        for line in input {
            let mut glyphs: Vec<_> = line.chars().map(|c| Glyph::Symbol(c)).collect();
            for m in r.find_iter(line) {
                let num = Rc::new(u32::from_str_radix(m.as_str(), 10).ok()?);

                for i in m.range() {
                    glyphs[i] = Glyph::Number(num.clone())
                }
            }

            assert!(width == 0 || glyphs.len() == width);
            width = width.max(glyphs.len());
            height += 1;

            grid.extend(glyphs);
        }

        Some(Self {
            width,
            height,
            grid,
        })
    }

    fn get1(&self, x: usize, y: usize) -> Option<&Glyph> {
        if x > 0 && y > 0 && x <= self.width && y <= self.height {
            let x = x - 1;
            let y = y - 1;

            Some(&self.grid[y * self.width + x])
        } else {
            None
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let grid = Grid::parse(input).unwrap();

    let mut seen = HashSet::new();
    let mut sum = 0;

    for y in 1..=grid.height {
        for x in 1..=grid.width {
            match grid.get1(x, y) {
                Some(Glyph::Symbol(c)) if *c != '.' => {}
                _ => continue,
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let x = (x as isize + dx) as usize;
                    let y = (y as isize + dy) as usize;

                    if let Some(Glyph::Number(num)) = grid.get1(x, y) {
                        if seen.insert(Rc::as_ptr(num)) {
                            sum += num.as_ref();
                        }
                    }
                }
            }
        }
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let grid = Grid::parse(input).unwrap();

    let mut sum = 0;

    for y in 1..=grid.height {
        for x in 1..=grid.width {
            match grid.get1(x, y) {
                Some(Glyph::Symbol('*')) => {}
                _ => continue,
            }

            let mut numbers = HashSet::new();

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let x = (x as isize + dx) as usize;
                    let y = (y as isize + dy) as usize;

                    if let Some(Glyph::Number(num)) = grid.get1(x, y) {
                        numbers.insert(Rc::as_ptr(num));
                    }
                }
            }

            if numbers.len() == 2 {
                sum += numbers
                    .into_iter()
                    .fold(1, |acc, n| acc * unsafe { n.as_ref().unwrap() })
            }
        }
    }

    sum
}

impl_dayx!("03", solve1, solve2);
