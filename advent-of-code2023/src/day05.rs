use std::collections::BTreeMap;
use std::ops::Bound;

type Map = BTreeMap<usize, (usize, usize)>;

struct Input {
    seeds: Vec<usize>,

    maps: [Map; 7],
}

impl Input {
    fn parse(lines: &[String]) -> Option<Self> {
        let (_, seeds) = lines.get(0)?.split_once("seeds: ")?;
        let seeds: Vec<usize> = seeds
            .split(' ')
            .map(|s| usize::from_str_radix(s.trim(), 10))
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        let mut maps = Vec::new();

        for line in lines.into_iter().skip(2) {
            if line.is_empty() {
                continue;
            }
            if line.ends_with("map:") {
                maps.push(Map::new());
                continue;
            }

            let map = maps.last_mut().unwrap();
            let mut split = line.splitn(3, ' ');
            let dst = usize::from_str_radix(split.next()?.trim(), 10).ok()?;
            let src = usize::from_str_radix(split.next()?.trim(), 10).ok()?;
            let count = usize::from_str_radix(split.next()?.trim(), 10).ok()?;

            map.insert(src, (dst, count));
        }

        Some(Self {
            seeds,
            maps: maps.try_into().ok()?,
        })
    }

    fn location_for_seed(&self, seed: usize) -> usize {
        let mut it = seed;

        for map in self.maps.iter() {
            if let Some((&begin, &(dst, count))) = map.range(..=it).next_back() {
                if it < begin + count {
                    let index = it - begin;
                    it = dst + index;
                }
            }
        }

        it
    }
}

fn solve1(input: &[String]) -> impl std::fmt::Display {
    let input = Input::parse(input).unwrap();

    let mut min = usize::MAX;

    for seed in input.seeds.iter() {
        min = min.min(input.location_for_seed(*seed));
    }

    min
}

fn solve2(input: &[String]) -> impl std::fmt::Display {
    let input = Input::parse(input).unwrap();
    assert_eq!(input.seeds.len() % 2, 0);

    let mut seed_ranges = Vec::new();
    for i in 0..input.seeds.len() / 2 {
        let start = input.seeds[i * 2];
        let end = start + input.seeds[i * 2 + 1];

        seed_ranges.push(start..end);
    }

    let mut rev_maps: [Map; 7] = input
        .maps
        .iter()
        .map(|m| {
            m.iter()
                .map(|(&begin, &(dst, count))| (dst, (begin, count)))
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // insert the 1:1 ranges
    for rm in rev_maps.iter_mut() {
        let mut to_insert = Vec::new();
        let mut prev = 0;

        for (&dst, &(_begin, count)) in rm.iter() {
            if dst > prev {
                to_insert.push((prev, (prev, dst - prev)));
            }
            prev = dst + count;
        }

        if usize::MAX > prev {
            to_insert.push((prev, (prev, usize::MAX - prev)));
        }

        rm.extend(to_insert);
    }

    let mut stack = vec![(
        rev_maps.len() - 1,
        0..usize::MAX,
        rev_maps.last().unwrap().upper_bound(Bound::Included(&0)),
    )];

    while let Some((i, range, iter)) = stack.last_mut() {
        if let Some((&dst, &(begin, count))) = iter.key_value() {
            iter.move_next();
            if dst + count <= range.start {
                continue;
            }

            let offset = if range.start > dst {
                range.start - dst
            } else {
                0
            };
            let end = (dst + count).min(range.end);
            if end <= dst {
                stack.pop();
                continue;
            }

            let new_begin = begin + offset;
            let new_count = end - dst - offset;
            let range = new_begin..new_begin + new_count;

            if *i == 0 {
                for seed_range in seed_ranges.iter() {
                    let overlap = range.start.max(seed_range.start);
                    if overlap < range.end && overlap < seed_range.end {
                        return input.location_for_seed(overlap);
                    }
                }
            } else {
                let i = *i - 1;

                stack.push((i, range, rev_maps[i].upper_bound(Bound::Included(&begin))));
            }
        } else {
            stack.pop();
        }
    }

    usize::MAX
}

impl_dayx!("05", solve1, solve2);
