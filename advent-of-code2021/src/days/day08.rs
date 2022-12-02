use super::read_lines;

// I was hoping for type checks but sadly the compiler will not complain :(
type Bits6 = u32;
type Bits2 = u32;
type Bits3 = u32;
type Bits5 = u32;
type Bits4 = u32;
type Bits7 = u32;

#[derive(Debug, Clone)]
enum Config {
    Zero(Bits6),
    One(Bits2),
    Two(Bits5),
    Three(Bits5),
    Four(Bits4),
    Five(Bits5),
    Six(Bits6),
    Seven(Bits3),
    Eight(Bits7),
    Nine(Bits6),

    TwoThreeFive(Bits5),
    ZeroSixNine(Bits6),

    ThreeFive(Bits5),
    TwoFive(Bits5),

    ZeroSix(Bits6),
    ZeroNine(Bits6),
}

impl TryFrom<&str> for Config {
    type Error = ();

    fn try_from(string: &str) -> Result<Config, Self::Error> {
        let chars = string
            .bytes()
            .map(|x| x + 1 - 97u8)
            .fold(0, |acc, x| acc | (1 << x)) as u32;

        match string.len() {
            2 => Ok(Config::One(chars as Bits2)),
            3 => Ok(Config::Seven(chars as Bits3)),
            4 => Ok(Config::Four(chars as Bits4)),
            5 => Ok(Config::TwoThreeFive(chars as Bits5)),
            6 => Ok(Config::ZeroSixNine(chars as Bits6)),
            7 => Ok(Config::Eight(chars as Bits7)),
            _ => Err(()),
        }
    }
}

pub fn solve() {
    let mut observations = Vec::new();
    let mut outputs = Vec::new();

    for line in read_lines("inputs/d08/0.txt")
        .expect("Could not find input for day 8!")
        .flatten()
    {
        let (observation, output) = line.split_once('|').expect("Invalid format!");

        let observation: [Config; 10] = observation
            .trim()
            .split(' ')
            .map(|v| v.try_into().expect("Invalid config!"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Invalid format!");

        let output: [Config; 4] = output
            .trim()
            .split(' ')
            .map(|v| v.try_into().expect("Invalid config!"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Invalid format!");

        observations.push(observation);
        outputs.push(output);
    }

    println!("Day 08 - First:");
    solve_first(&outputs);
    println!("Day 08 - Secnd:");
    solve_scnd(&observations, &outputs);
}

fn solve_first(outputs: &[[Config; 4]]) {
    let count = outputs
        .iter()
        .flatten()
        .filter(|config| !matches!(config, Config::TwoThreeFive(_) | Config::ZeroSixNine(_)))
        .count();

    println!("The answer is {}", count);
}

fn count_common(a: u32, b: u32) -> u32 {
    (a & b).count_ones()
}

fn configs_to_number(configs: &[Config]) -> Option<u32> {
    let mut number = 0;
    for config in configs {
        number *= 10;
        match config {
            Config::Zero(_) => {}
            Config::One(_) => number += 1,
            Config::Two(_) => number += 2,
            Config::Three(_) => number += 3,
            Config::Four(_) => number += 4,
            Config::Five(_) => number += 5,
            Config::Six(_) => number += 6,
            Config::Seven(_) => number += 7,
            Config::Eight(_) => number += 8,
            Config::Nine(_) => number += 9,
            _ => return None,
        }
    }

    Some(number)
}

macro_rules! unwrap_enum {
    ($obj:expr) => {
        match ($obj) {
            Config::Zero(x) => x,
            Config::One(x) => x,
            Config::Two(x) => x,
            Config::Three(x) => x,
            Config::Four(x) => x,
            Config::Five(x) => x,
            Config::Six(x) => x,
            Config::Seven(x) => x,
            Config::Eight(x) => x,
            Config::Nine(x) => x,

            Config::TwoThreeFive(x) => x,
            Config::ZeroSixNine(x) => x,

            Config::ThreeFive(x) => x,
            Config::TwoFive(x) => x,

            Config::ZeroSix(x) => x,
            Config::ZeroNine(x) => x,
        }
    };
}

macro_rules! find_matching {
    ($iter:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        $iter.find(|x| matches!(x, $( $pattern )|+ $( if $guard )?)).map(|m| unwrap_enum!(m))
    };
}

fn deduce_one(observation: &[Config; 10], output: &[Config; 4]) -> u32 {
    let mut resolved = output.to_owned();

    loop {
        let mut fails = 0;

        for i in 0..resolved.len() {
            match resolved[i] {
                Config::TwoThreeFive(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::One(_))
                    {
                        match count_common(*m, v) {
                            1 => resolved[i] = Config::TwoFive(v),
                            2 => resolved[i] = Config::Three(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Four(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::Two(v),
                            3 => resolved[i] = Config::ThreeFive(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Seven(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::TwoFive(v),
                            3 => resolved[i] = Config::Three(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                Config::ZeroSixNine(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::One(_))
                    {
                        match count_common(*m, v) {
                            1 => resolved[i] = Config::Six(v),
                            2 => resolved[i] = Config::ZeroNine(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Four(_))
                    {
                        match count_common(*m, v) {
                            3 => resolved[i] = Config::ZeroSix(v),
                            4 => resolved[i] = Config::Nine(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Seven(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::Six(v),
                            3 => resolved[i] = Config::ZeroNine(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                Config::ThreeFive(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::One(_))
                    {
                        match count_common(*m, v) {
                            1 => resolved[i] = Config::Five(v),
                            2 => resolved[i] = Config::Three(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Seven(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::Five(v),
                            3 => resolved[i] = Config::Three(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                Config::TwoFive(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Four(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::Two(v),
                            3 => resolved[i] = Config::Five(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                Config::ZeroNine(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Four(_))
                    {
                        match count_common(*m, v) {
                            3 => resolved[i] = Config::Zero(v),
                            4 => resolved[i] = Config::Nine(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                Config::ZeroSix(v) => {
                    if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::One(_))
                    {
                        match count_common(*m, v) {
                            1 => resolved[i] = Config::Six(v),
                            2 => resolved[i] = Config::Zero(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else if let Some(m) =
                        find_matching!(resolved.iter().chain(observation), Config::Seven(_))
                    {
                        match count_common(*m, v) {
                            2 => resolved[i] = Config::Six(v),
                            3 => resolved[i] = Config::Zero(v),
                            _ => unreachable!("logic error!"),
                        }
                    } else {
                        fails += 1;
                    }
                }
                _ => {}
            };
        }

        if let Some(number) = configs_to_number(&resolved) {
            return number;
        }

        if fails == 4 {
            println!("{:?} - {:?}", observation, output);
            println!("{:?}", resolved);
            panic!("[Infinite loop] Could not deduce number!");
        }
    }
}

fn solve_scnd(observations: &[[Config; 10]], outputs: &[[Config; 4]]) {
    let mut sum = 0;
    for i in 0..observations.len() {
        sum += deduce_one(&observations[i], &outputs[i]);
    }

    println!("The answer is {}", sum);
}
