use std::collections::{HashMap, HashSet};

use regex::Regex;

enum Insn {
    Left,
    Right,
}

struct Map {
    instructions: Vec<Insn>,
    nodes: HashMap<u32, (u32, u32)>,
}

impl<T: AsRef<str>> TryFrom<&[T]> for Map {
    type Error = &'static str;

    fn try_from(value: &[T]) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();

        let instructions = iter
            .next()
            .ok_or("missing instructions")?
            .as_ref()
            .chars()
            .map(|c| match c {
                'L' => Ok(Insn::Left),
                'R' => Ok(Insn::Right),
                _ => Err("invalid direction"),
            })
            .collect::<Result<Vec<Insn>, _>>()?;

        // skip new line
        iter.next();

        let mut nodes = HashMap::new();
        let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap();

        let mut tmp = [0; 4];

        for line in iter {
            let line = line.as_ref();
            let m = re.captures(line).ok_or("invalid line")?;

            tmp[1..4].copy_from_slice(m.get(1).unwrap().as_str().as_bytes());
            let key = u32::from_be_bytes(tmp);

            tmp[1..4].copy_from_slice(m.get(2).unwrap().as_str().as_bytes());
            let left = u32::from_be_bytes(tmp);

            tmp[1..4].copy_from_slice(m.get(3).unwrap().as_str().as_bytes());
            let right = u32::from_be_bytes(tmp);

            nodes.insert(key, (left, right));
        }

        Ok(Self {
            instructions,
            nodes,
        })
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let map: Map = input.try_into().unwrap();

    const START_NODE: u32 = 0x414141;
    const DEST_NODE: u32 = 0x5a5a5a;

    let mut current = START_NODE;
    let mut steps = 0;

    for insn in map.instructions.iter().cycle() {
        if current == DEST_NODE {
            break;
        }

        steps += 1;

        let node = map.nodes.get(&current).unwrap();

        current = match insn {
            Insn::Left => node.0,
            Insn::Right => node.1,
        }
    }

    steps
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    use crate::util::lcm;
    use itertools::Itertools;

    let map: Map = input.try_into().unwrap();

    let mut all_steps = Vec::new();
    for id in map.nodes.keys() {
        if (id & 0xFF) != 0x41 {
            continue;
        }

        let mut current = *id;
        let mut seen = HashSet::new();

        let mut steps = Vec::new();
        let mut step_count = 0u64;
        for (ii, insn) in map.instructions.iter().enumerate().cycle() {
            if !seen.insert((ii, current)) {
                break;
            }

            step_count += 1;

            let node = map.nodes.get(&current).unwrap();
            current = match insn {
                Insn::Left => node.0,
                Insn::Right => node.1,
            };

            if (current & 0xFF) == 0x5a {
                steps.push(step_count);
            }
        }

        all_steps.push(steps);
    }

    let steps = all_steps
        .into_iter()
        .multi_cartesian_product()
        .map(|steps| steps.into_iter().fold(1, |acc, x| lcm(acc, x)))
        .min()
        .unwrap();

    steps
}

impl_dayx!("08", solve1, solve2);
