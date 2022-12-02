use super::read_lines;
use std::u32;

pub fn solve() {
    let lines = read_lines("inputs/d03/0.txt").expect("Could not find input for day 3!");
    let values: Vec<String> = lines.filter_map(|line| line.ok()).collect();

    println!("Day 03 - First:");
    solve_first(&values);
    println!("Day 03 - Secnd:");
    solve_second(&values);
}

fn solve_first(values: &[String]) {
    let mut eps = 0;
    let mut gamma = 0;

    if let Some(first) = values.first() {
        let mut one_counts = vec![0; first.len()];

        for val in values {
            if val.len() > one_counts.len() {
                one_counts.resize(val.len(), 0);
            }

            for (i, bit) in val.chars().enumerate() {
                if bit == '1' {
                    one_counts[i] += 1;
                }
            }
        }

        for i in 0..one_counts.len() {
            let count = one_counts[one_counts.len() - 1 - i];

            if count >= values.len() / 2 {
                gamma += 1 << i;
            } else {
                eps += 1 << i;
            }
        }
    }

    println!("The answer is {}", eps * gamma);
}

fn get_rating(values: &[String], number_of_bits: usize, test_bit: char) -> u32 {
    let mut filtered_values = values.to_owned();

    for current_bit in 0..number_of_bits {
        let mut one_count = 0;

        for val in filtered_values.iter() {
            if let Some('1') = val.chars().nth(current_bit) {
                one_count += 1;
            }
        }

        let bit_to_keep = if one_count >= filtered_values.len() - one_count {
            test_bit
        } else if test_bit == '1' {
            '0'
        } else {
            '1'
        };

        filtered_values.retain(|v| Some(bit_to_keep) == v.chars().nth(current_bit));

        if filtered_values.len() == 1 {
            return u32::from_str_radix(filtered_values.first().unwrap(), 2)
                .expect("Invalid input");
        }
    }

    panic!("Invalid input");
}

fn solve_second(values: &[String]) {
    let number_of_bits = values
        .iter()
        .map(|v| v.len())
        .max()
        .expect("Require at least one input!");

    let oxygen_generator_rating = get_rating(values, number_of_bits, '1');
    let co2_scrubber_rating = get_rating(values, number_of_bits, '0');

    println!(
        "The answer is {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}
