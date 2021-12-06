use super::read_lines;

pub fn solve() {
    let initial_fish_values: Vec<usize> = read_lines("inputs/d06/0.txt")
        .expect("Could not find input for day 6!")
        .flat_map(|line| {
            line.expect("Error while reading file.")
                .split(',')
                .map(|digit| digit.parse::<usize>().expect("Invalid input!"))
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut fish = [0u64; 9];
    for value in initial_fish_values {
        fish[value] += 1;
    }

    println!("Day 06 - First:");
    solve_x(fish, 80);
    println!("Day 06 - Secnd:");
    solve_x(fish, 256);
}

fn solve_x(mut fish: [u64; 9], days: usize) {
    for _ in 0..days {
        let next_gen = fish[0];
        for i in 1..9 {
            fish[i - 1] = fish[i];
        }
        fish[8] = next_gen;
        fish[6] += next_gen;
    }

    println!("The answer is {}", fish.iter().sum::<u64>());
}
