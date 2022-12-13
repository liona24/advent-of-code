#[derive(Debug, Clone, Copy)]
enum Insn {
    Nop,
    Add(i64),
}

impl TryFrom<&str> for Insn {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((_, arg)) = value.split_once(' ') {
            let arg = arg.parse::<i64>().map_err(|e| e.to_string())?;
            Ok(Insn::Add(arg))
        } else {
            Ok(Insn::Nop)
        }
    }
}

#[derive(Debug)]
struct Executor<'i> {
    instructions: &'i [Insn],
    i: usize,
    value: i64,
}

impl<'i> Executor<'i> {
    fn new(instructions: &'i [Insn]) -> Self {
        Self {
            instructions,
            i: 0,
            value: 1,
        }
    }
}

impl<'i> Iterator for Executor<'i> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.instructions.len() {
            None
        } else {
            match self.instructions[self.i] {
                Insn::Add(x) => {
                    self.value += x;
                }
                Insn::Nop => {}
            };

            self.i += 1;
            Some(self.value)
        }
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let mut instructions = Vec::new();

    for line in input {
        let insn: Insn = line.as_str().try_into().expect("invalid input");

        match insn {
            Insn::Add(_) => {
                instructions.push(Insn::Nop);
                instructions.push(insn);
            }
            Insn::Nop => {
                instructions.push(Insn::Nop);
            }
        }
    }

    let mut sum = 0;

    let executor = Executor::new(&instructions);
    for (i, value) in executor.into_iter().enumerate() {
        let i = i + 2; // one for zero based and one for "during"

        if i == 20 || i == 60 || i == 100 || i == 140 || i == 180 || i == 220 {
            sum += i as i64 * value;
        }

        if i >= 220 {
            break;
        }
    }

    sum
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    // stall one to bring crt in sync ..
    let mut instructions = vec![Insn::Nop];

    for line in input {
        let insn: Insn = line.as_str().try_into().expect("invalid input");

        match insn {
            Insn::Add(_) => {
                instructions.push(Insn::Nop);
                instructions.push(insn);
            }
            Insn::Nop => {
                instructions.push(Insn::Nop);
            }
        }
    }

    let mut display = ['.' as u8; 6 * 40];

    let executor = Executor::new(&instructions);
    for (i, value) in executor.into_iter().enumerate() {
        let crt = (i % 40) as i64;
        if crt - 1 <= value && crt + 1 >= value {
            display[i] = '#' as u8;
        }
    }

    let lines: Vec<String> = display
        .chunks(40)
        .map(|chunk| "\n".to_string() + String::from_utf8_lossy(chunk).as_ref())
        .collect();

    lines.join("")
}

impl_dayx!("10", solve1, solve2);
