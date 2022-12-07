use std::collections::BTreeMap;

use regex::Regex;

fn change_dir<'a>(dirs: &mut Vec<&'a str>, dir: &'a str) {
    if dir.starts_with('/') {
        dirs.clear();
        dirs.push(dir);
    } else if dir.starts_with("..") {
        dirs.pop();
    } else {
        dirs.push(dir);
    }
}

fn calc_dirs_sizes(input: &[String]) -> BTreeMap<String, usize> {
    let mut dirs = Vec::new();
    let mut dirs_sizes = BTreeMap::new();

    let re_cd = Regex::new(r"\$ cd (.+)$").expect("invalid regex");
    let re_file = Regex::new(r"(\d+) (.+)").expect("invalid regex");

    for line in input {
        if let Some(captures) = re_cd.captures(line) {
            change_dir(&mut dirs, captures.get(1).unwrap().as_str());
        } else if let Some(captures) = re_file.captures(line) {
            assert!(!dirs.is_empty());

            let size = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

            for i in 1..=dirs.len() {
                let dir = dirs[..i].join("/");
                *dirs_sizes.entry(dir).or_insert(0) += size;
            }
        }
    }

    dirs_sizes
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let dirs_sizes = calc_dirs_sizes(input);

    dirs_sizes.values().filter(|&&v| v <= 100000).sum::<usize>()
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let dirs_sizes = calc_dirs_sizes(input);

    let total = 70000000;
    let needed = 30000000;
    let used = *dirs_sizes.get("/").expect("invalid input");
    let min = needed - (total - used);

    *dirs_sizes
        .values()
        .filter(|&&v| v >= min)
        .min()
        .expect("should find something")
}

impl_dayx!("07", solve1, solve2);
