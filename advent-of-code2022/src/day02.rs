#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl TryFrom<&str> for Choice {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().next() {
            Some('A') | Some('X') => Ok(Self::Rock),
            Some('B') | Some('Y') => Ok(Self::Paper),
            Some('C') | Some('Z') => Ok(Self::Scissors),
            _ => Err("invalid input"),
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().next() {
            Some('X') => Ok(Self::Lose),
            Some('Y') => Ok(Self::Draw),
            Some('Z') => Ok(Self::Win),
            _ => Err("invalid input"),
        }
    }
}

impl Choice {
    fn score_vs(&self, other: Choice) -> usize {
        match (*self, other) {
            (Self::Rock, Self::Rock) => 1 + 3,
            (Self::Rock, Self::Paper) => 1 + 0,
            (Self::Rock, Self::Scissors) => 1 + 6,

            (Self::Paper, Self::Rock) => 2 + 6,
            (Self::Paper, Self::Paper) => 2 + 3,
            (Self::Paper, Self::Scissors) => 2 + 0,

            (Self::Scissors, Self::Rock) => 3 + 0,
            (Self::Scissors, Self::Paper) => 3 + 6,
            (Self::Scissors, Self::Scissors) => 3 + 3,
        }
    }

    fn select_for_outcome(&self, outcome: Outcome) -> Self {
        match (*self, outcome) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Lose) => Self::Scissors,

            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Lose) => Self::Rock,

            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Lose) => Self::Paper,

            (x, Outcome::Draw) => x,
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input {
        let pair = line.split_once(' ').expect("invalid input");

        let you: Choice = pair.0.try_into().expect("invalid input");
        let me: Choice = pair.1.try_into().expect("invalid input");

        score += me.score_vs(you);
    }

    score
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input {
        let pair = line.split_once(' ').expect("invalid input");

        let you: Choice = pair.0.try_into().expect("invalid input");
        let outcome: Outcome = pair.1.try_into().expect("invalid input");

        let me = you.select_for_outcome(outcome);

        score += me.score_vs(you);
    }

    score
}

impl_dayx!("02", solve1, solve2);
