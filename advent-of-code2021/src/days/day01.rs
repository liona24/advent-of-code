use super::read_lines;

pub fn solve() {
    let lines = read_lines("inputs/d01/0.txt").expect("Could not find input for day 1!");
    let values: Vec<u32> = lines
        .filter_map(|line| line.ok().map(|line| line.parse().expect("Invalid input!")))
        .collect();

    println!("Day 01 - First:");
    solve_first(&values);
    println!("Day 02 - Secnd:");
    solve_second(&values);
}

fn solve_first(values: &[u32]) {
    let mut prev = None;
    let mut ans = 0;

    for val in values {
        if let Some(prev_val) = prev {
            if prev_val < val {
                ans += 1;
            }
        }

        prev = Some(val);
    }

    println!("The answer is {}", ans);
}

fn solve_second(values: &[u32]) {
    let mut agg = vec![0];
    for v in values {
        agg.push(agg.last().unwrap() + (*v as u64));
    }

    const SLICE_SIZE: usize = 3;

    let mut prev = None;
    let mut ans = 0;

    for i in SLICE_SIZE..=values.len() {
        let sum = agg[i] - agg[i - SLICE_SIZE];

        if let Some(prev) = prev {
            if sum > prev {
                ans += 1;
            }
        }

        prev = Some(sum);
    }

    println!("The answer is {}", ans);
}
