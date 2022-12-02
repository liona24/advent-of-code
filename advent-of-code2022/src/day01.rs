fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut calories = vec![0u32];
    for line in input {
        if let Some(count) = line.parse::<u32>().ok() {
            *calories.last_mut().unwrap() += count;
        } else {
            calories.push(0);
        }
    }

    calories.into_iter().max().unwrap()
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut calories = vec![0u32];
    for line in input {
        if let Some(count) = line.parse::<u32>().ok() {
            *calories.last_mut().unwrap() += count;
        } else {
            calories.push(0);
        }
    }

    calories.sort();

    calories.into_iter().rev().take(3).sum::<u32>()
}

impl_dayx!("01", solve1, solve2);
