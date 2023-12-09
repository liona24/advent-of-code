use std::num::ParseIntError;

fn parse_history(s: impl AsRef<str>) -> Result<Vec<i32>, ParseIntError> {
    let s = s.as_ref();

    s.split(' ')
        .filter(|x| !x.is_empty())
        .map(|num| i32::from_str_radix(num, 10))
        .collect()
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;

    for line in input {
        let mut history = parse_history(line).unwrap();

        let mut len = history.len();

        loop {
            for i in 1..len {
                let prev = history[i - 1];
                let it = history[i];

                history[i - 1] = it - prev;
            }

            len -= 1;

            if history[..len].iter().all(|x| *x == 0) {
                let result = history[len..].iter().sum::<i32>();
                sum += result;
                break;
            }
        }
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;

    for line in input {
        let mut history = parse_history(line).unwrap();

        let mut start = 1;

        loop {
            for i in (start..history.len()).rev() {
                let prev = history[i - 1];
                let it = history[i];

                history[i] = it - prev;
            }

            if history[start..].iter().all(|x| *x == 0) {
                let result = history[..start].iter().rev().fold(0, |acc, x| x - acc);
                sum += result;
                break;
            }

            start += 1;
        }
    }

    sum
}

impl_dayx!("09", solve1, solve2);
