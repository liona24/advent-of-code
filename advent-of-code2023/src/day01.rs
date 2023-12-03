fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut sum = 0;
    for line in input {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();

        let mut num = String::new();
        num.push(first);
        num.push(last);

        sum += u64::from_str_radix(&num, 10).unwrap();
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    use regex::Regex;

    let r = Regex::new(r"(0)|((?:one)|1)|((?:two)|2)|((?:three)|3)|((?:four)|4)|((?:five)|5)|((?:six)|6)|((?:seven)|7)|((?:eight)|8)|((?:nine)|9)").unwrap();

    let mut sum = 0;

    for line in input {
        let first = r.captures(line).unwrap();
        let (d0, _) = first
            .iter()
            .skip(1)
            .enumerate()
            .find(|x| x.1.is_some())
            .unwrap();

        for i in (0..line.len()).rev() {
            if let Some(secnd) = r.captures_at(line, i) {
                let (d1, _) = secnd
                    .iter()
                    .skip(1)
                    .enumerate()
                    .find(|x| x.1.is_some())
                    .unwrap();

                sum += d0 * 10 + d1;
                break;
            }
        }
    }

    sum
}

impl_dayx!("01", solve1, solve2);
