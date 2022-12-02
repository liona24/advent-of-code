use super::read_lines;
use std::collections::HashMap;

const EOF: char = '$';

#[derive(Debug)]
struct Polymer {
    value: char,
    next: Option<Box<Self>>,
}

impl From<&str> for Polymer {
    fn from(string: &str) -> Self {
        let mut chars = string.chars();

        let mut polymer = Polymer {
            value: chars.next().expect("Empty sequence!"),
            next: None,
        };

        let mut prev = &mut polymer;
        for chr in chars {
            prev.next = Some(Box::new(Polymer {
                value: chr,
                next: None,
            }));

            prev = prev.next.as_mut().unwrap();
        }

        polymer
    }
}

impl Polymer {
    fn histogram(&self) -> HashMap<char, u64> {
        let mut hist = HashMap::new();

        let mut current_opt = Some(self);

        while let Some(current) = current_opt {
            hist.entry(current.value)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            current_opt = current.next.as_deref();
        }

        hist
    }

    fn as_2gram(&self) -> HashMap<[char; 2], u64> {
        let mut grams = HashMap::new();

        let mut current_opt = Some(self);

        while let Some(current) = current_opt {
            let gram = [
                current.value,
                current.next.as_ref().map_or(EOF, |next| next.value),
            ];

            grams.entry(gram).and_modify(|v| *v += 1).or_insert(1);

            current_opt = current.next.as_deref();
        }

        grams
    }
}

pub fn solve() {
    let mut lines = read_lines("inputs/d14/0.txt")
        .expect("Could not find input for day 14!")
        .filter_map(|line| line.ok());

    let template: Polymer = lines.next().expect("No template found!").as_str().into();
    let template = Box::new(template);

    let mut rules = HashMap::new();

    let polymer2 = template.as_2gram();

    // Skipt empty line
    lines.next();

    for line in lines {
        let (pat, rep) = line.split_once(" -> ").expect("Invalid rule!");

        let pat: [char; 2] = pat
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .expect("Invalid pattern!");
        let rep = rep.chars().next().expect("Invalid replacement!");

        rules.insert(pat, rep);
    }

    println!("Day 14 - First:");
    solve_first(template, &rules);
    println!("Day 14 - Secnd:");
    solve_secnd(polymer2, &rules);
}

fn solve_first(mut template: Box<Polymer>, rules: &HashMap<[char; 2], char>) {
    for _ in 0..10 {
        let mut current_opt = Some(&mut template);
        while let Some(mut current) = current_opt {
            if current.next.is_none() {
                break;
            }

            let next_value = current.next.as_ref().map(|polymer| polymer.value).unwrap();

            let pat = [current.value, next_value];

            if let Some(rep) = rules.get(&pat) {
                let old_next = std::mem::replace(
                    current.next.as_mut().unwrap(),
                    Box::new(Polymer {
                        value: *rep,
                        next: None,
                    }),
                );

                current = current.next.as_mut().unwrap();
                current.next = Some(old_next);
            }

            current_opt = current.next.as_mut();
        }
    }

    let hist = template.histogram();
    let (_, most) = hist.iter().max_by_key(|kv| kv.1).expect("Empty polymer?");
    let (_, least) = hist.iter().min_by_key(|kv| kv.1).expect("Empty polymer?");

    println!("The answer is {}", most - least);
}

fn solve_secnd(mut polymer: HashMap<[char; 2], u64>, rules: &HashMap<[char; 2], char>) {
    let mut next_polymer = HashMap::new();

    for _ in 0..40 {
        for (pat, count) in polymer.drain() {
            if let Some(&rep) = rules.get(&pat) {
                let pat1 = [pat[0], rep];
                let pat2 = [rep, pat[1]];

                next_polymer
                    .entry(pat1)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                next_polymer
                    .entry(pat2)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            } else {
                next_polymer
                    .entry(pat)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        }

        std::mem::swap(&mut polymer, &mut next_polymer);
    }

    let mut hist = HashMap::new();
    for (pat, count) in polymer.into_iter() {
        hist.entry(pat[0])
            .and_modify(|v| *v += count)
            .or_insert(count);
    }

    let (_, most) = hist.iter().max_by_key(|kv| kv.1).expect("Empty polymer?");
    let (_, least) = hist.iter().min_by_key(|kv| kv.1).expect("Empty polymer?");

    println!("The answer is {}", most - least);
}
