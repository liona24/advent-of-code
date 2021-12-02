use super::read_lines;

enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl TryFrom<&str> for Command {
    type Error = ();

    fn try_from(string: &str) -> Result<Command, Self::Error> {
        if let Some((cmd, arg)) = string.split_once(" ") {
            let arg: u32 = arg.parse().map_err(|_| ())?;

            match cmd {
                "forward" => Ok(Command::Forward(arg)),
                "down" => Ok(Command::Down(arg)),
                "up" => Ok(Command::Up(arg)),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

pub fn solve() {
    let lines = read_lines("inputs/d02/0.txt").expect("Could not find input for day 2!");
    let values: Vec<Command> = lines
        .filter_map(|line| {
            line.ok()
                .map(|line| line.as_str().try_into().expect("Invalid input!"))
        })
        .collect();

    println!("Day 02 - First:");
    solve_first(&values);
    println!("Day 02 - Secnd:");
    solve_second(&values);
}

fn solve_first(values: &[Command]) {
    let mut x = 0;
    let mut depth = 0;

    for val in values {
        match val {
            Command::Up(dist) => {
                depth -= dist;
            }
            Command::Down(dist) => {
                depth += dist;
            }
            Command::Forward(dist) => {
                x += dist;
            }
        }
    }

    println!("The answer is {}", x * depth);
}

fn solve_second(values: &[Command]) {
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    for val in values {
        match val {
            Command::Up(dist) => {
                aim -= dist;
            }
            Command::Down(dist) => {
                aim += dist;
            }
            Command::Forward(dist) => {
                x += dist;
                depth += aim * dist;
            }
        }
    }

    println!("The answer is {}", x * depth);
}
