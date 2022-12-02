use super::read_lines;

pub fn solve() {
    let mut crab_positions: Vec<usize> = read_lines("inputs/d07/0.txt")
        .expect("Could not find input for day 7!")
        .flat_map(|line| {
            line.expect("Error while reading file.")
                .split(',')
                .map(|digit| digit.parse::<usize>().expect("Invalid input!"))
                .collect::<Vec<usize>>()
        })
        .collect();
    crab_positions.sort_unstable();

    println!("Day 07 - First:");
    solve_first(&crab_positions);
    println!("Day 07 - Secnd:");
    solve_secnd(&crab_positions);
}

fn solve_first(crab_positions: &[usize]) {
    let med = crab_positions[crab_positions.len() / 2];

    println!(
        "The answer is {}",
        crab_positions
            .iter()
            .map(|x| (*x as i64 - med as i64).abs())
            .sum::<i64>()
    );
}

fn get_cost2(crab_positions: &[usize], target: usize) -> usize {
    crab_positions
        .iter()
        .map(|x| {
            let diff = (*x as i64 - target as i64).abs() as usize;
            diff * (diff + 1) / 2
        })
        .sum()
}

fn solve_secnd(crab_positions: &[usize]) {
    let mut best_cost = get_cost2(crab_positions, 0);

    for i in 1..=*crab_positions
        .iter()
        .max()
        .expect("Should have at least one input!")
    {
        let cost = get_cost2(crab_positions, i);

        if cost < best_cost {
            best_cost = cost;
        }
    }

    println!("The answer is {}", best_cost);
}
