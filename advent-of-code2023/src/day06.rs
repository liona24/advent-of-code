struct Pair {
    time: usize,
    distance: usize,
}

fn parse_input1(input: &[String]) -> Option<Vec<Pair>> {
    let (_, times) = input.get(0)?.split_once("Time:")?;
    let (_, distances) = input.get(1)?.split_once("Distance:")?;

    let times = times
        .split(' ')
        .filter(|split| !split.is_empty())
        .map(|s| usize::from_str_radix(s, 10));
    let distances = distances
        .split(' ')
        .filter(|split| !split.is_empty())
        .map(|s| usize::from_str_radix(s, 10));

    times
        .zip(distances)
        .map(|(t, d)| {
            t.and_then(|t| {
                d.and_then(|d| {
                    Ok(Pair {
                        time: t,
                        distance: d,
                    })
                })
            })
        })
        .collect::<Result<Vec<Pair>, _>>()
        .ok()
}

fn parse_input2(input: &[String]) -> Option<Pair> {
    let (_, times) = input.get(0)?.split_once("Time:")?;
    let (_, distances) = input.get(1)?.split_once("Distance:")?;

    let time = usize::from_str_radix(times.replace(' ', "").as_str(), 10).ok()?;
    let distance = usize::from_str_radix(distances.replace(' ', "").as_str(), 10).ok()?;

    Some(Pair { time, distance })
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let pairs = parse_input1(input).unwrap();

    let mut res = 1u64;
    for pair in pairs {
        let mut possibilities = 0;
        for t in pair.distance / pair.time..pair.time {
            let v = t;
            let d = v * (pair.time - t);
            if d > pair.distance {
                possibilities += 1;
            } else if possibilities > 0 {
                break;
            }
        }

        res *= possibilities;
    }

    res
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let pair = parse_input2(input).unwrap();

    let mut possibilities = 0usize;
    for t in pair.distance / pair.time..pair.time {
        let v = t;
        let d = v * (pair.time - t);
        if d > pair.distance {
            possibilities += 1;
        } else if possibilities > 0 {
            break;
        }
    }

    possibilities
}

impl_dayx!("06", solve1, solve2);
