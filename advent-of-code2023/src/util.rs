use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

macro_rules! impl_dayx {
    ($day:expr, $solve1:ident, $solve2:ident) => {
        pub fn solve() {
            let lines: Vec<String> = crate::util::read_lines(&format!("inputs/{}.txt", $day))
                .expect(&format!("could not find input for day {}", $day))
                .filter_map(|res| res.ok())
                .collect();
            println!("Day {} - First: {}", $day, $solve1(&lines));
            println!("       - Secnd: {}", $solve2(&lines));
        }
    };
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}
