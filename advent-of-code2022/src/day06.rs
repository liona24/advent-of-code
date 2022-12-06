#[derive(Debug, Clone)]
struct SmallMultiSet {
    counts: [usize; 256],
    duplicates: usize,
}

impl SmallMultiSet {
    fn new() -> Self {
        Self {
            counts: [0; 256],
            duplicates: 0,
        }
    }

    fn add(&mut self, value: u8) {
        self.counts[value as usize] += 1;
        if self.counts[value as usize] == 2 {
            self.duplicates += 1;
        }
    }

    fn remove(&mut self, value: u8) {
        self.counts[value as usize] -= 1;
        if self.counts[value as usize] == 1 {
            self.duplicates -= 1;
        }
    }

    fn has_duplicates(&self) -> bool {
        self.duplicates > 0
    }
}

fn solvex(input: &str, n: usize) -> i64 {
    let mut buf = SmallMultiSet::new();

    for c in input.chars().take(n) {
        buf.add(c as u8);
    }

    if !buf.has_duplicates() {
        return 0;
    }

    for (old, (i, new)) in input.chars().zip(input.chars().enumerate().skip(n)) {
        buf.remove(old as u8);
        buf.add(new as u8);

        if !buf.has_duplicates() {
            return i as i64 + 1;
        }
    }

    -1
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let input = input.first().expect("invalid input");

    solvex(input, 4)
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let input = input.first().expect("invalid input");

    solvex(input, 14)
}

impl_dayx!("06", solve1, solve2);
