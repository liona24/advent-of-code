use regex::Regex;

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn apply_to(&self, stacks: &mut [(usize, Vec<char>)]) {
        for _ in 0..self.count {
            let tmp = stacks[self.from].1.pop().expect("invalid input");
            stacks[self.to].1.push(tmp);
        }
    }
    fn apply_to2(&self, stacks: &mut [(usize, Vec<char>)]) {
        let mut tmp = Vec::new();
        for _ in 0..self.count {
            tmp.push(stacks[self.from].1.pop().expect("invalid input"));
        }
        stacks[self.to].1.extend(tmp.iter().rev());
    }
}

fn parse_input_or_die(input: &[String]) -> (Vec<(usize, Vec<char>)>, Vec<Move>) {
    let stacks_re = Regex::new(r"^( \d )+").expect("invalid regex");
    let moves_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("invalid regex");

    let mut stacks: Option<Vec<(usize, Vec<char>)>> = None;
    let mut moves: Vec<Move> = Vec::new();
    for line in input.iter().rev() {
        if stacks_re.is_match(line) {
            let mut tmp = Vec::new();
            for (i, c) in line.char_indices() {
                if c.is_ascii_digit() {
                    tmp.push((i, Vec::new()));
                }
            }

            stacks = Some(tmp);
        } else if let Some(captures) = moves_re.captures(line) {
            let count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

            moves.push(Move { count, from, to });
        } else if let Some(stacks) = stacks.as_mut() {
            for stack in stacks {
                let index = stack.0;
                assert!(index < line.len());

                let c = line.as_bytes()[index] as char;
                if c.is_ascii_alphabetic() {
                    stack.1.push(c)
                }
            }
        }
    }

    if stacks.is_none() {
        panic!("invalid input!");
    }
    moves.reverse();

    (stacks.unwrap(), moves)
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let (mut stacks, moves) = parse_input_or_die(input);

    for m in moves {
        m.apply_to(&mut stacks);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.1.last().cloned())
        .collect::<String>()
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let (mut stacks, moves) = parse_input_or_die(input);

    for m in moves {
        m.apply_to2(&mut stacks);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.1.last().cloned())
        .collect::<String>()
}

impl_dayx!("05", solve1, solve2);
