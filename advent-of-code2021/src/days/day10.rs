use super::read_lines;

pub fn solve() {
    let inputs: Vec<String> = read_lines("inputs/d10/0.txt")
        .expect("Could not find input for day 10!")
        .filter_map(|line| line.ok())
        .collect();

    println!("Day 10 - First:");
    solve_first(&inputs);
    println!("Day 10 - Secnd:");
    solve_secnd(&inputs);
}

fn solve_first(inputs: &[String]) {
    let mut score = 0;

    for input in inputs {
        let mut open = Vec::new();
        for chr in input.bytes() {
            match (chr, open.last()) {
                (b'(', _) | (b'[', _) | (b'{', _) | (b'<', _) => open.push(chr),
                (b')', Some(b'('))
                | (b']', Some(b'['))
                | (b'}', Some(b'{'))
                | (b'>', Some(b'<')) => {
                    open.pop();
                }
                (b')', _) => {
                    score += 3;
                    break;
                }
                (b']', _) => {
                    score += 57;
                    break;
                }
                (b'}', _) => {
                    score += 1197;
                    break;
                }
                (b'>', _) => {
                    score += 25137;
                    break;
                }
                _ => unreachable!("Invalid input!"),
            };
        }
    }

    println!("The answer is {}", score);
}

fn solve_secnd(inputs: &[String]) {
    let mut scores = Vec::new();

    for input in inputs {
        let mut open = Vec::new();
        let mut is_wrong = false;

        for chr in input.bytes() {
            match (chr, open.last()) {
                (b'(', _) | (b'[', _) | (b'{', _) | (b'<', _) => open.push(chr),
                (b')', Some(b'('))
                | (b']', Some(b'['))
                | (b'}', Some(b'{'))
                | (b'>', Some(b'<')) => {
                    open.pop();
                }
                (b')', _) | (b']', _) | (b'}', _) | (b'>', _) => {
                    is_wrong = true;
                    break;
                }
                _ => unreachable!("Invalid input!"),
            };
        }

        if !is_wrong && !open.is_empty() {
            let mut score: u64 = 0;

            for chr in open.iter().rev() {
                score *= 5;
                match chr {
                    b'(' => score += 1,
                    b'[' => score += 2,
                    b'{' => score += 3,
                    b'<' => score += 4,
                    _ => unreachable!(),
                }
            }

            scores.push(score);
        }
    }

    scores.sort_unstable();

    println!("The answer is {}", scores[scores.len() / 2]);
}
