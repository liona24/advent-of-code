use super::read_lines;
use std::collections::{BTreeMap, BTreeSet};

enum Fold {
    X(u32),
    Y(u32),
}

pub fn solve() {
    let lines = read_lines("inputs/d13/0.txt")
        .expect("Could not find input for day 13!")
        .filter_map(|line| line.ok());

    let mut xmap = BTreeMap::new();
    let mut ymap = BTreeMap::new();
    let mut folds = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("fold") {
            let instruction = line.split(' ').last().expect("Invalid input!");
            let (axis, value) = instruction.split_once('=').expect("Invalid input!");
            let value = value.parse::<u32>().expect("Should be number!");

            match axis {
                "x" => folds.push(Fold::X(value)),
                "y" => folds.push(Fold::Y(value)),
                _ => panic!("Invalid input!"),
            }
        } else {
            let (x, y) = line.split_once(',').expect("Invalid input!");

            let x = x.parse::<u32>().expect("Should be number!");
            let y = y.parse::<u32>().expect("Should be number!");

            xmap.entry(x).or_insert_with(BTreeSet::new).insert(y);
            ymap.entry(y).or_insert_with(BTreeSet::new).insert(x);
        }
    }

    println!("Day 13 - First:");
    solve_first(&mut xmap, &mut ymap, &folds);
    println!("Day 13 - Secnd:");
    solve_secnd(&mut xmap, &mut ymap, &folds);
}

fn do_fold(
    v: u32,
    refmap: &mut BTreeMap<u32, BTreeSet<u32>>,
    fixmap: &mut BTreeMap<u32, BTreeSet<u32>>,
) {
    let to_fold = refmap.split_off(&v);
    for (r, mut fset) in to_fold.into_iter() {
        if r == v {
            for f in fset {
                fixmap.entry(f).and_modify(|values| {
                    values.remove(&r);
                });
            }
        } else {
            let new_r = 2 * v - r;

            for f in fset.iter() {
                let values = fixmap.get_mut(f).unwrap();
                values.remove(&r);
                values.insert(new_r);
            }

            refmap
                .entry(new_r)
                .and_modify(|values| {
                    values.append(&mut fset);
                })
                .or_insert(fset);
        }
    }
}

fn solve_first(
    xmap: &mut BTreeMap<u32, BTreeSet<u32>>,
    ymap: &mut BTreeMap<u32, BTreeSet<u32>>,
    folds: &[Fold],
) {
    if let Some(fold) = folds.first() {
        match fold {
            Fold::X(x) => do_fold(*x, xmap, ymap),
            Fold::Y(y) => do_fold(*y, ymap, xmap),
        }
    }

    println!(
        "The answer is {}",
        xmap.values().map(|set| set.len()).sum::<usize>()
    );
}

fn solve_secnd(
    xmap: &mut BTreeMap<u32, BTreeSet<u32>>,
    ymap: &mut BTreeMap<u32, BTreeSet<u32>>,
    folds: &[Fold],
) {
    for fold in folds.iter().skip(1) {
        match fold {
            Fold::X(x) => do_fold(*x, xmap, ymap),
            Fold::Y(y) => do_fold(*y, ymap, xmap),
        }
    }

    let x_max = *xmap.keys().max().unwrap() as usize + 1;
    let y_max = *ymap.keys().max().unwrap() as usize + 1;

    let mut display = vec![vec![' '; x_max]; y_max];
    for (&x, ys) in xmap.iter() {
        for &y in ys.iter() {
            display[y as usize][x as usize] = '#';
        }
    }

    for line in display.into_iter() {
        for chr in line.into_iter() {
            print!("{}", chr);
        }
        println!();
    }
}
