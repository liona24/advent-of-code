use std::collections::BTreeSet;

fn priority_for_item(item: &u8) -> usize {
    match *item {
        97..=122 => (*item - 96) as usize,
        65..=90 => (*item - 64) as usize + 26,
        _ => 0,
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;
    for line in input {
        let items = line.trim().as_bytes();

        let split = items.len() / 2;
        let first: BTreeSet<u8> = items[..split].iter().cloned().collect();
        let secnd: BTreeSet<u8> = items[split..].iter().cloned().collect();

        score += first
            .intersection(&secnd)
            .map(priority_for_item)
            .sum::<usize>()
    }

    score
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let mut score = 0;

    for group in input.chunks(3) {
        let items1 = group[0].trim().as_bytes();
        let items2 = group[1].trim().as_bytes();
        let items3 = group[2].trim().as_bytes();

        let items1: BTreeSet<u8> = items1.iter().cloned().collect();
        let items2: BTreeSet<u8> = items2.iter().cloned().collect();
        let items3: BTreeSet<u8> = items3.iter().cloned().collect();

        let items12: BTreeSet<u8> = items1.intersection(&items2).cloned().collect();

        score += items12
            .intersection(&items3)
            .map(priority_for_item)
            .sum::<usize>()
    }

    score
}

impl_dayx!("03", solve1, solve2);
