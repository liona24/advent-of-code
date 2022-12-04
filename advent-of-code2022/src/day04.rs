#[derive(Debug, Clone)]
struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && other.high <= self.high
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.low >= other.low && self.low <= other.high)
            || (self.high >= other.low && self.high <= other.high)
            || self.contains(other)
            || other.contains(self)
    }
}

impl TryFrom<&str> for Range {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (low, high) = value.split_once('-').ok_or("invalid format".to_string())?;
        let low = low.parse::<usize>().map_err(|e| e.to_string())?;
        let high = high.parse::<usize>().map_err(|e| e.to_string())?;

        Ok(Self { low, high })
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input {
        let pair = line.split_once(',').expect("invalid input");

        let first: Range = pair.0.try_into().expect("invalid input");
        let secnd: Range = pair.1.try_into().expect("invalid input");

        if first.contains(&secnd) || secnd.contains(&first) {
            score += 1
        }
    }

    score
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input {
        let pair = line.split_once(',').expect("invalid input");

        let first: Range = pair.0.try_into().expect("invalid input");
        let secnd: Range = pair.1.try_into().expect("invalid input");

        if first.overlaps(&secnd) {
            score += 1
        }
    }

    score
}

impl_dayx!("04", solve1, solve2);
